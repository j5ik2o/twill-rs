use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;
use crate::core::parser_monad::ParserMonad;
use crate::core::ParseError;
use crate::core::rc_parser::to_rc_parser;

/// Trait providing parser operators
pub trait OperatorParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + Sized 
where
  Self: 'a, {
  /// Apply parsers selectively (disjunction) with lazy alternative evaluation
  fn or<P>(self, alt: P) -> impl Parser<'a, I, A>
  where
    P: Parser<'a, I, A>, {
    self.or_with(|| alt)
  }

  fn or_with<F, P>(self, f: F) -> impl Parser<'a, I, A>
  where
    F: FnOnce() -> P,
    P: Parser<'a, I, A>, {
    move |parse_context: ParseContext<'a, I>| match self.parse(parse_context) {
      pr @ ParseResult::Failure {
        committed_status: CommittedStatus::Uncommitted,
        ..
      } => {
        let alt = f();
        alt.parse(pr.context().with_same_state())
      }
      other => other,
    }
  }

  /// Sequential parser that uses a function to create the second parser
  fn and_then_with<F, P2, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: FnOnce(A) -> P2,
    P2: Parser<'a, I, B>, {
    self.flat_map(f)
  }

  /// Sequential parser (conjunction) - implemented using flat_map and map (no Clone required)
  fn and_then<P2, B>(self, p2: P2) -> impl Parser<'a, I, (A, B)>
  where
    P2: Parser<'a, I, B>, {
    self.and_then_with(move |a| p2.map(move |b| (a, b)))
  }

  /// Negation parser - succeeds when self fails, fails when self succeeds
  fn not(self) -> impl Parser<'a, I, ()> {
    move |parse_context: ParseContext<'a, I>| match self.parse(parse_context) {
      ParseResult::Success { parse_context, .. } => {
        let len = parse_context.last_offset().unwrap_or(0);
        let parser_error = ParseError::of_mismatch(parse_context, len, "not predicate failed".to_string());
        ParseResult::failed_with_uncommitted(parser_error)
      }
      pr @ ParseResult::Failure { .. } => ParseResult::successful(pr.context().with_same_state(), (), 0),
    }
  }

  /// Sequential parser with lazy evaluation (discard first parser result) - implemented using flat_map
  fn skip_left_with<F, P2, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: FnOnce() -> P2,
    P2: Parser<'a, I, B>, {
    self.flat_map(move |_| f())
  }

  /// Sequential parser (discard first parser result) - implemented using skip_left_with
  fn skip_left<P2, B>(self, p2: P2) -> impl Parser<'a, I, B>
  where
    P2: Parser<'a, I, B>, {
    self.skip_left_with(move || p2)
  }

  /// Sequential parser with lazy evaluation (discard second parser result) - implemented using flat_map
  fn skip_right_with<F, P2>(self, f: F) -> impl Parser<'a, I, A>
  where
    F: FnOnce() -> P2,
    P2: Parser<'a, I, ()>, {
    self.flat_map(move |a| f().map(move |_| a))
  }

  /// Sequential parser (discard second parser result) - implemented using skip_right_with
  fn skip_right<P2>(self, p2: P2) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, ()>, {
    self.skip_right_with(move || p2)
  }

  /// Discard the result and return ()
  fn discard(self) -> impl Parser<'a, I, ()> {
    self.map(|_| ())
  }

  /// Transforms any failure into an uncommitted failure
  /// This allows the parser to be used in an or_with operation even if it would normally commit
  fn attempt(self) -> impl Parser<'a, I, A> {
    move |parse_context: ParseContext<'a, I>| self.parse(parse_context).with_uncommitted()
  }

  fn scan_right1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + Clone + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a,
    Self: Clone, {
    // ここでもRcParserを使用してクローン制約を回避
    let rc_parser = to_rc_parser(self);
    let op_clone = op.clone();
    
    move |parse_context: ParseContext<'a, I>| {
      match rc_parser.clone().parse(parse_context) {
        ParseResult::Success { parse_context, value, length: _ } => {
          let next_parser = rc_parser.clone().rest_right1(op_clone, value);
          next_parser.parse(parse_context)
        }
        parse_result @ ParseResult::Failure { .. } => parse_result,
      }
    }
  }

  fn chain_left1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + Clone + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a,
    Self: Clone, {
    let rc_parser = to_rc_parser(self);
    let op_clone = op.clone();
    
    move |parse_context: ParseContext<'a, I>| {
      match rc_parser.clone().parse(parse_context) {
        ParseResult::Success { parse_context, value, length: _ } => {
          let op_clone_fn = move || op_clone.clone();
          let next_parser = rc_parser.clone().rest_left1(op_clone_fn, value);
          next_parser.parse(parse_context)
        }
        parse_result @ ParseResult::Failure { .. } => parse_result,
      }
    }
  }

  fn rest_right1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + Clone + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a,
    Self: Clone, {
    let rc_parser = to_rc_parser(self);
    let default_value = x.clone();
    op.flat_map(move |f| {
      let default_value = x.clone();
      rc_parser.clone().map(move |y| f(default_value, y))
    })
    .or(move |pc: ParseContext<'a, I>| ParseResult::successful(pc, default_value.clone(), 0))
  }

  /// Left associative binary operator parsing with default value
  ///
  /// This method takes an operator parser and a default value, and
  /// returns a parser that repeatedly applies the left associative operation on
  /// the parsed values, or returns the default value if no operations can be applied.
  fn rest_left1<P2, OP, F>(self, op: F, default_value: A) -> impl Parser<'a, I, A>
  where
    F: Fn() -> P2 + 'a,
    P2: Parser<'a, I, OP>,
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a,
    Self: Clone, {
    // Wrap the original parser in an RcParser to make it cloneable
    let rc_parser = to_rc_parser(self);
    let value_parser = move |ctx: ParseContext<'a, I>| rc_parser.clone().parse(ctx);

    move |parse_context: ParseContext<'a, I>| {
      // Parse the initial value
      let initial_result = value_parser(parse_context);

      match initial_result {
        // If the initial value could not be parsed, return the default value
        ParseResult::Failure { error, .. } => {
          ParseResult::successful(error.parse_context().with_same_state(), default_value.clone(), 0)
        }
        // If the initial value was parsed, repeatedly apply operators and next values
        ParseResult::Success {
          parse_context: mut ctx,
          value: mut left_value,
          length: mut total_length,
        } => {
          // Repeatedly parse the remaining operators and values
          loop {
            // Parse the operator
            let op_result = op().parse(ctx.with_same_state());
            if let ParseResult::Success {
              parse_context: op_ctx,
              value: operator,
              length: op_length,
            } = op_result
            {
              // Parse the next value
              let right_result = value_parser(op_ctx.advance(op_length));
              if let ParseResult::Success {
                parse_context: new_ctx,
                value: right_value,
                length: right_length,
              } = right_result
              {
                // Apply the operator to update the result (FnOnce is applied directly)
                left_value = operator(left_value, right_value);
                ctx = new_ctx.advance(right_length);
                total_length += op_length + right_length;
                continue;
              }
            }
            // Break the loop if parsing fails
            break;
          }

          // Return the result
          ParseResult::successful(ctx, left_value, total_length)
        }
      }
    }
  }
}

impl<'a, T, I: 'a, A> OperatorParser<'a, I, A> for T 
where 
  T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
