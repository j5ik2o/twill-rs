use crate::parse_result::ParseResult;
use crate::parser::{successful, Parser, RcParser};

/// Trait providing parser transformation methods (consuming self)
pub trait ParserMonad<'a, I: 'a, A>: Parser<'a, I, A> {
  /// Transform success result (consuming self)
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B> // Changed to take self
  where
    // Self: 'a, // No longer needed as self is consumed
    A: 'a,
    B: Clone + 'a,
    F: Fn(A) -> B + 'a, { // F still needs 'a if it captures refs
    // Call flat_map which also takes self
    self.flat_map(move |a| successful(f(a)))
  }

  /// Chain parsers (consuming self)
  fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B> // Changed to take self
  where
    // Self: 'a, // No longer needed as self is consumed
    A: 'a,
    B: 'a,
    P: Parser<'a, I, B> + 'a,
    F: Fn(A) -> P + 'a, { // F still needs 'a if it captures refs
    // self is moved into the closure
    RcParser::new(move |parse_context| match self.run(parse_context) { // self.run takes &self, so this works
      ParseResult::Success {
        parse_context,
        value,
        length,
      } => f(value).run(parse_context.advance(length)),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserMonad<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
