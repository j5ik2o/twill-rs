use crate::combinators::successful;
use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{ParserRunner, Parser};

/// Trait providing parser transformation methods (consuming self)
pub trait ParserMonad<'a, I: 'a, A>: ParserRunner<'a, I, A> {
  /// Transform success result (consuming self)
  fn map<F, B>(self, f: F) -> Parser<'a, I, B, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a>
  where
    A: 'a,
    B: Clone + 'a,
    F: Fn(A) -> B + 'a, {
    self.flat_map(move |a| successful(f(a)))
  }

  /// Chain parsers (consuming self)
  fn flat_map<F, P, B>(self, f: F) -> Parser<'a, I, B, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a>
  where
    A: 'a,
    B: 'a,
    P: ParserRunner<'a, I, B> + 'a,
    F: Fn(A) -> P + 'a, {
    Parser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        parse_context,
        value: a,
        length: n,
      } => {
        log::debug!("1) parse_context.offset = {}", parse_context.next_offset());
        let ps = parse_context.add_offset(n);
        let result = f(a).run(ps).with_committed_fallback(n != 0).with_add_length(n);
        log::debug!("2) parse_context.offset = {}", parse_context.next_offset());
        result
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
impl<'a, T, I: 'a, A> ParserMonad<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  fn test_map() {
    let text: &str = "a";
    let input = text.chars().collect::<Vec<_>>();
    let p = elm_ref('a').map(|_| 'b');

    let result = p.parse(&input).to_result();
    println!("{:?}", result);

    assert!(result.is_ok());
  }

  #[test]
  fn test_flat_map() {
    let text: &str = "a";
    let input = text.chars().collect::<Vec<_>>();
    let p = elm_ref('a').flat_map(|_| successful('b'));

    let result = p.parse(&input).to_result();
    println!("{:?}", result);

    assert!(result.is_ok());
  }
}
