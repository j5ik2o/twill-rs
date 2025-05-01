use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::{FuncParser, Parser};

/// Trait providing parser transformation methods
pub trait ParserMonad<'a, I: 'a, A>: Parser<'a, I, A> + Sized + Clone {
  /// Transform success result
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    Self: 'a,
      B: Clone + 'a,
    F: Fn(A) -> B + Clone + 'a, {
    FuncParser::new(
      move |parse_context: ParseContext<'a, I>| match self.clone().run(parse_context) {
        ParseResult::Success {
          parse_context,
          value,
          length,
        } => ParseResult::successful(parse_context, f(value), length),
        ParseResult::Failure {
          error,
          committed_status,
        } => ParseResult::failed(error, committed_status),
      },
    )
  }

  /// Chain parsers
  fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    Self: 'a,
      B: Clone + 'a,
    P: Parser<'a, I, B> + 'a,
    F: Fn(A) -> P + Clone + 'a, {
    FuncParser::new(
      move |parse_context: ParseContext<'a, I>| match self.clone().run(parse_context) {
        ParseResult::Success {
          parse_context, value, ..
        } => f(value).run(parse_context),
        ParseResult::Failure {
          error,
          committed_status,
        } => ParseResult::failed(error, committed_status),
      },
    )
  }

  /// Filter parser results based on a predicate
  fn with_filter<F>(self, f: F) -> impl Parser<'a, I, A>
  where
    Self: 'a,
      A: Clone + 'a,
    F: Fn(&A) -> bool + Clone + 'a, {
    FuncParser::new(
      move |parse_context: ParseContext<'a, I>| match self.clone().run(parse_context) {
        ParseResult::Success {
          parse_context,
          value,
          length,
        } => {
          if f(&value) {
            ParseResult::successful(parse_context, value, length)
          } else {
            let message = "Filter condition not satisfied".to_string();
            let error =
              crate::core::parse_error::ParseError::of_mismatch(parse_context.with_same_state(), length, message);
            ParseResult::failed_with_uncommitted(error)
          }
        }
        failed => failed,
      },
    )
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserMonad<'a, I, A> for T where T: Parser<'a, I, A> + Clone {}
