use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// Trait providing parser transformation methods
pub trait ParserExt<'a, I: 'a, A>: Parser<'a, I, A> + Sized {
  /// Transform success result
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: FnOnce(A) -> B, {
    move |input: ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Success { value, context } => ParseResult::successful(f(value), context),
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
    move |input: ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Success { value, context } => f(value).parse(context),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserExt<'a, I, A> for T where T: Parser<'a, I, A> {}
