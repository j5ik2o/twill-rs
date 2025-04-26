use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// Trait providing parser transformation methods
pub trait ParserExt<'a, I: 'a, A>: Parser<'a, I, A> + Sized {
  /// Transform success result
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: Fn(A) -> B, {
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { value, length } => 
        ParseResult::successful(f(value), length),
      ParseResult::Failure { error, committed_status } => 
        ParseResult::failed(error, committed_status),
    }
  }

  /// Chain parsers
  fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: Fn(A) -> P,
    P: Parser<'a, I, B>, {
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { value, length } => {
        let remaining = &input[length..];
        f(value).parse(remaining).with_add_length(length)
      },
      ParseResult::Failure { error, committed_status } => 
        ParseResult::failed(error, committed_status),
    }
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserExt<'a, I, A> for T where T: Parser<'a, I, A> {}
