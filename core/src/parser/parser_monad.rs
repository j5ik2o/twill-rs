use std::fmt::Debug;
use crate::combinators::successful;
use crate::parse_context::ParseContext;
use crate::parse_error::ParseError;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, ParserRunner};

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
        log::debug!("1) parse_context.next_offset = {}", parse_context.next_offset());
        let ps = parse_context.add_offset(n);
        let result = f(a).run(ps).with_committed_fallback(n != 0).with_add_length(n);
        log::debug!("2) parse_context.next_offset = {}", parse_context.next_offset());
        result
      }
      ParseResult::Failure {
        parse_context,
        error,
        committed_status: is_committed,
      } => ParseResult::failed(parse_context, error, is_committed),
    })
  }

  /// 解析結果をフィルターする[Parser]を返す。
  fn with_filter<F>(self, f: F) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
      F: Fn(&A) -> bool + 'a,
      I: Debug + 'a,
      A: 'a,
      Self: Sized {
    Parser::new(move |parse_state| match self.run(parse_state.with_same_state()) {
      ParseResult::Success { parse_context, value, length, .. } => {
        if f(&value) {
          ParseResult::successful(parse_context, value, length)
        } else {
          let input = parse_state.input();
          let offset = parse_state.last_offset().unwrap_or(0);
          let msg = format!("no matched to predicate: last offset: {}", offset);
          let ps = parse_state.add_offset(length);
          let pe = ParseError::of_mismatch(input, ps.next_offset(), length, msg);
          ParseResult::failed_with_uncommitted(parse_context, pe)
        }
      }
      ParseResult::Failure {
        parse_context,
        error,
        committed_status: is_committed,
      } => ParseResult::failed(parse_context, error, is_committed),
    })
  }

  /// 解析結果をフィルターする[Parser]を返す。
  fn with_filter_not<F>(self, f: F) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
      F: Fn(&A) -> bool + 'a,
      I: Debug + 'a,
      A: 'a,
      Self: Sized, {
    self.with_filter(move |e| !f(e))
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
