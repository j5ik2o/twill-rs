use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::or_parser::OrParser;
use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::rc_parser::to_rc_parser;
use crate::core::parser::{FuncParser, Parser};
use crate::core::{successful, RcParser};

/// Trait providing binary operator related parser operations
pub trait BinaryOperatorParser<'a, I: 'a, A>:
  Parser<'a, I, A> + ParserMonad<'a, I, A> + OrParser<'a, I, A> + Sized
where
  Self: 'a, {
  /// Right associative binary operator parsing
  fn chain_right1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    let rc_parser = to_rc_parser(self);
    rc_parser.clone().flat_map(move |x| rc_parser.rest_right1(op, x))
  }

  /// Left associative binary operator parsing
  fn chain_left1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    let rc_parser = to_rc_parser(self);
    rc_parser.clone().flat_map(move |x| rc_parser.rest_left1(op, x))
  }

  /// Right associative binary operator parsing helper
  fn rest_right1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    let default_value = x.clone();
    op.flat_map(move |f| {
      let default_value = default_value.clone();
      self.map(move |y| f(y, default_value))
    })
    .or(successful(x, 0))
  }

  /// Left associative binary operator parsing helper with the default value
  ///
  /// This method takes an operator parser and a default value and
  /// returns a parser that repeatedly applies the left associative operation on
  /// the parsed values or returns the default value if no operations can be applied.
  fn rest_left1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    OP: FnOnce(A, A) -> A + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    fn rest_left0<'a, I, A, OP>(
      rc_parser: RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A>>,
      op_rc_parser: RcParser<'a, I, OP, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, OP>>,
      x: A,
    ) -> impl Parser<'a, I, A>
    where
      A: Clone + 'a,
      OP: FnOnce(A, A) -> A + 'a, {
      let default_value = x.clone();
      FuncParser::new(
        move |parse_context: ParseContext<'a, I>| match op_rc_parser.clone().run(parse_context) {
          ParseResult::Success {
            parse_context: mut pc1,
            value: f,
            length: n1,
          } => {
            pc1.advance_mut(n1);
            (match rc_parser.clone().run(pc1) {
              ParseResult::Success {
                parse_context: mut pc2,
                value: y,
                length: n2,
              } => {
                pc2.advance_mut(n2);
                rest_left0(rc_parser, op_rc_parser, f(y, default_value.clone()))
                  .run(pc2)
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
        },
      )
      .or(successful(x.clone(), 0))
    }

    rest_left0(to_rc_parser(self), to_rc_parser(op), x)
  }
}

/// Implement BinaryOperatorParser for all types that implement Parser, ParserMonad, ChoiceParser and Clone
impl<'a, T, I: 'a, A> BinaryOperatorParser<'a, I, A> for T where
  T: Parser<'a, I, A> + ParserMonad<'a, I, A> + OrParser<'a, I, A> + 'a
{
}
