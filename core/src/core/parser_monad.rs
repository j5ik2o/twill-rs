use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// Trait providing parser transformation methods
pub trait ParserMonad<'a, I: 'a, A>: Parser<'a, I, A> + Sized {
  /// Transform success result
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: FnOnce(A) -> B, {
    move |parse_context: ParseContext<'a, I>| match self.parse(parse_context) {
      ParseResult::Success { parse_context, value, length  } => ParseResult::successful(parse_context , f(value), length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }

  /// Chain parsers
  fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: FnOnce(A) -> P,
    P: Parser<'a, I, B>, {
    move |parse_context: ParseContext<'a, I>| match self.parse(parse_context) {
      ParseResult::Success { parse_context, value, .. } => f(value).parse(parse_context),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserMonad<'a, I, A> for T where T: Parser<'a, I, A> {}
