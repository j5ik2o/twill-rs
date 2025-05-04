use crate::parse_result::ParseResult;
use crate::parser::{successful, Parser, RcParser};

/// Trait providing parser transformation methods (consuming self)
pub trait ParserMonad<'a, I: 'a, A>: Parser<'a, I, A> {
  /// Transform success result (consuming self)
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    A: 'a,
    B: Clone + 'a,
    F: Fn(A) -> B + 'a, {
    self.flat_map(move |a| successful(f(a)))
  }

  /// Chain parsers (consuming self)
  fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    A: 'a,
    B: 'a,
    P: Parser<'a, I, B> + 'a,
    F: Fn(A) -> P + 'a, {
    RcParser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        parse_context: mut pc1,
        value,
        length,
      } => {
        pc1.advance_mut(length);
        f(value).run(pc1)
      }
      ParseResult::Failure {
        parse_context,
        error,
        committed_status,
      } => ParseResult::failed(parse_context, error, committed_status),
    })
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserMonad<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
