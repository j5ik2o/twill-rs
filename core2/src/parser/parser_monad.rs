use crate::combinators::successful;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, RcParser};

/// Trait providing parser transformation methods (consuming self)
pub trait ParserMonad<'a, I: 'a, A>: Parser<'a, I, A> {
  /// Transform success result (consuming self)
  #[inline(always)]
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    A: 'a,
    B: Clone + 'a,
    F: Fn(A) -> B + 'a, {
    self.flat_map(move |a| successful(f(a)))
  }

  /// Chain parsers (consuming self)
  #[inline(always)]
  fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    A: 'a,
    B: 'a,
    P: Parser<'a, I, B> + 'a,
    F: Fn(A) -> P + 'a, {
    RcParser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success { parse_context, value: a, length: n } => {
        let ps = parse_context.advance(n);
        f(a).run(ps).with_committed_fallback(n != 0).with_add_length(n)
      }
      ParseResult::Failure {
        parse_context,
        error,
        committed_status: is_committed,
      } => ParseResult::failed(parse_context, error, is_committed),
    })
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserMonad<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
