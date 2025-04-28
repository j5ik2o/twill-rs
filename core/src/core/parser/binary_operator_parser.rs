use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::choice_parser::ChoiceParser;
use crate::core::parser::rc_parser::{to_rc_parser};
use crate::core::parser::Parser;
use crate::core::parser_monad::ParserMonad;

/// Trait providing binary operator related parser operations
pub trait BinaryOperatorParser<'a, I: 'a, A>:
  Parser<'a, I, A> + ParserMonad<'a, I, A> + ChoiceParser<'a, I, A> + Sized
where
  Self: 'a, {
  /// Right associative binary operator parsing
  fn scan_right1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + 'a, // Clone constraint removed
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a, {
    let rc_parser = to_rc_parser(self);
    let op_rc = to_rc_parser(op);

    move |parse_context: ParseContext<'a, I>| match rc_parser.clone().parse(parse_context) {
      ParseResult::Success {
        parse_context,
        value,
        length: _,
      } => {
        let next_parser = rc_parser.clone().rest_right1(op_rc, value);
        next_parser.parse(parse_context)
      }
      parse_result @ ParseResult::Failure { .. } => parse_result,
    }
  }

  /// Left associative binary operator parsing
  fn chain_left1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + 'a, // This needs Clone - it's used multiple times
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a, {
    let rc_parser = to_rc_parser(self);
    let op_clone =  to_rc_parser(op);

    move |parse_context: ParseContext<'a, I>| match rc_parser.clone().parse(parse_context) {
      ParseResult::Success {
        parse_context,
        value,
        length: _,
      } => {
        let op_fn = move || op_clone.clone();
        let next_parser = rc_parser.clone().rest_left1(op_fn, value);
        next_parser.parse(parse_context)
      }
      parse_result @ ParseResult::Failure { .. } => parse_result,
    }
  }

  /// Right associative binary operator parsing helper
  fn rest_right1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + 'a, // Clone constraint removed
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a, {
    let rc_parser = to_rc_parser(self);
    let default_value = x.clone();

    // Wrap op in to_single_use_rc_parser so it can be consumed without cloning
    let mapped = op.flat_map(move |f| {
      let default_value = x.clone();
      rc_parser.clone().map(move |y| f(default_value, y))
    });

    move |pc: ParseContext<'a, I>| {
      // コンテキストの元の状態を取得する
      let original_pc = pc.with_same_state();

      // mappedにpcを使用
      let result = mapped.parse(pc);

      match result {
        ok @ ParseResult::Success { .. } => ok,
        _ => {
          // 失敗した場合は元の状態のコンテキストを使用して成功を返す
          ParseResult::successful(original_pc, default_value.clone(), 0)
        }
      }
    }
  }

  /// Left associative binary operator parsing helper with the default value
  ///
  /// This method takes an operator parser and a default value and
  /// returns a parser that repeatedly applies the left associative operation on
  /// the parsed values or returns the default value if no operations can be applied.
  fn rest_left1<P2, OP, F>(self, op: F, default_value: A) -> impl Parser<'a, I, A>
  where
    F: Fn() -> P2 + 'a,
    P2: Parser<'a, I, OP>,
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a, {
    // Wrap the original parser in an RcParser to make it cloneable
    let rc_parser = to_rc_parser(self);

    move |parse_context: ParseContext<'a, I>| {
      // Parse the initial value
      let initial_result = rc_parser.clone().parse(parse_context);

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
              let right_result = rc_parser.clone().parse(op_ctx.advance(op_length));
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

/// Implement BinaryOperatorParser for all types that implement Parser, ParserMonad, ChoiceParser and Clone
impl<'a, T, I: 'a, A> BinaryOperatorParser<'a, I, A> for T where
  T: Parser<'a, I, A> + ParserMonad<'a, I, A> + ChoiceParser<'a, I, A> + Clone + 'a
{
}
