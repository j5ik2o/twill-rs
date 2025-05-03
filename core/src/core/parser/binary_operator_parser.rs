use crate::core::parse_result::ParseResult;
use crate::core::parser::reusable_with_clone;
use crate::core::parser::OrParser;
use crate::core::parser::ParserMonad;
use crate::core::parser::RcParser;
use crate::core::parser::{ClonableParser, FnParser};
use crate::core::{ParseContext, Parser};
use crate::prelude::successful;

/// Trait providing binary operator related parser operations
pub trait BinaryOperatorParser<'a, I: 'a, A>:
  ClonableParser<'a, I, A> + ParserMonad<'a, I, A> + OrParser<'a, I, A>
where
  Self: 'a, {
  /// Right associative binary operator parsing
  fn chain_right1<P2, OP>(self, op: P2) -> impl ClonableParser<'a, I, A>
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: ClonableParser<'a, I, OP> + 'a, {
    self.clone().flat_map(move |x| self.clone().rest_right1(op.clone(), x))
  }

  /// Left associative binary operator parsing
  fn chain_left1<P2, OP>(self, op: P2) -> impl ClonableParser<'a, I, A>
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: ClonableParser<'a, I, OP> + 'a, {
    self.clone().flat_map(move |x| self.clone().rest_left1(op.clone(), x))
  }

  /// Right associative binary operator parsing helper
  fn rest_right1<P2, OP>(self, op: P2, x: A) -> impl ClonableParser<'a, I, A>
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: ClonableParser<'a, I, OP> + 'a, {
    let default_value = x.clone();
    op.clone()
      .flat_map(move |f| {
        let default_value = x.clone();
        self.clone().map(move |y| f(default_value.clone(), y))
      })
      .or(successful(default_value.clone()))
  }

  /// Left associative binary operator parsing helper with the default value
  ///
  /// This method takes an operator parser and a default value and
  /// returns a parser that repeatedly applies the left associative operation on
  /// the parsed values or returns the default value if no operations can be applied.
  fn rest_left1<P2, OP>(self, op: P2, x: A) -> impl ClonableParser<'a, I, A>
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: ClonableParser<'a, I, OP> + 'a, {
    fn rest_left0<'a, I, A, OP>(
      rc_parser: RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A>>,
      op_rc_parser: RcParser<'a, I, OP, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, OP>>,
      x: A,
    ) -> impl ClonableParser<'a, I, A>
    where
      A: Clone + 'a,
      OP: Fn(A, A) -> A + Clone + 'a, {
      let default_value = x.clone();
      FnParser::new(move |parse_context| match op_rc_parser.run(parse_context) {
        ParseResult::Success {
          parse_context: mut pc1,
          value: f,
          length: n1,
        } => {
          pc1.advance_mut(n1);
          (match rc_parser.run(pc1) {
            ParseResult::Success {
              parse_context: mut pc2,
              value: y,
              length: n2,
            } => {
              pc2.advance_mut(n2);
              rest_left0(rc_parser.clone(), op_rc_parser.clone(), f(y, default_value.clone()))
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
      })
      .or(successful(x.clone()))
    }

    let parser_cloned = reusable_with_clone(self);
    let op_cloned = reusable_with_clone(op);
    rest_left0(parser_cloned, op_cloned, x)
  }
}

/// Implement BinaryOperatorParser for all types that implement Parser, ParserMonad, ChoiceParser and Clone
impl<'a, T, I: 'a, A> BinaryOperatorParser<'a, I, A> for T where
  T: ClonableParser<'a, I, A> + ParserMonad<'a, I, A> + OrParser<'a, I, A> + 'a
{
}
