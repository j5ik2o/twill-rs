use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::choice_parser::ChoiceParser;
use crate::core::parser::rc_parser::to_rc_parser;
use crate::core::parser::Parser;
use crate::core::parser_monad::ParserMonad;
use crate::core::{successful, RcParser};

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

    move |parse_context: ParseContext<'a, I>| match rc_parser.clone().parse(parse_context) {
      ParseResult::Success {
        parse_context,
        value,
        length: _,
      } => {
        let next_parser = rc_parser.rest_right1(op, value);
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

    move |parse_context: ParseContext<'a, I>| match rc_parser.clone().parse(parse_context) {
      ParseResult::Success {
        parse_context,
        value,
        length: _,
      } => {
        let next_parser = rc_parser.rest_left1(op, value);
        next_parser.parse(parse_context)
      }
      parse_result @ ParseResult::Failure { .. } => parse_result,
    }
  }

  /// Right associative binary operator parsing helper
  fn rest_right1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a, {
    let default_value = x.clone();

    // Wrap op in to_single_use_rc_parser so it can be consumed without cloning
    let mapped = op.flat_map(move |f| {
      let default_value = x.clone();
      self.map(move |y| f(default_value, y))
    });

    move |pc: ParseContext<'a, I>| {
      let original_pc = pc.with_same_state();
      let result = mapped.parse(pc);
      match result {
        ok @ ParseResult::Success { .. } => ok,
        _ => {
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
  fn rest_left1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, OP> + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    A: Clone + std::fmt::Debug + 'a, {
    let rc_parser = to_rc_parser(self);
    let op_rc_parser = to_rc_parser(op);
    rc_rest_left(rc_parser, op_rc_parser, x)
  }
}

/// Implement BinaryOperatorParser for all types that implement Parser, ParserMonad, ChoiceParser and Clone
impl<'a, T, I: 'a, A> BinaryOperatorParser<'a, I, A> for T where
  T: Parser<'a, I, A> + ParserMonad<'a, I, A> + ChoiceParser<'a, I, A> + Clone + 'a
{
}

fn rc_rest_left<'a, I, A, OP>(
  rc_parser: RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A>>,
  op_rc_parser: RcParser<'a, I, OP, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, OP>>,
  x: A,
) -> impl Parser<'a, I, A>
where
  I: 'a,
  OP: FnOnce(A, A) -> A + 'a,
  A: Clone + std::fmt::Debug + 'a, {
  let default_value = x.clone();
  (move |parse_context: ParseContext<'a, I>| match op_rc_parser.clone().parse(parse_context) {
    ParseResult::Success {
      parse_context: mut pc1,
      value: f,
      length: n1,
    } => {
      pc1.advance_mut(n1);
      (match rc_parser.clone().parse(pc1) {
        ParseResult::Success {
          parse_context: mut pc2,
          value: y,
          length: n2,
        } => {
          pc2.advance_mut(n2);
          rc_rest_left(rc_parser, op_rc_parser, f(y, default_value.clone()))
            .parse(pc2)
            .with_add_length(n2)
        }
        ParseResult::Failure {
          error,
          committed_status,
        } => ParseResult::failed(error, committed_status),
      })
      .with_committed_fallback(n1 != 0)
      .with_add_length(n1)
    }
    ParseResult::Failure {
      error,
      committed_status,
    } => ParseResult::failed(error, committed_status),
  })
  .or(successful(x.clone(), 0))
}
