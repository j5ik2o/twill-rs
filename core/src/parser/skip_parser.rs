use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::and_then_parser::AndThenParser;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{ParserRunner, Parser};
use std::ops::{Mul, Sub};

/// Trait providing sequence-related parser operations (consuming self)
pub trait SkipParser<'a, I: 'a, A>: ParserRunner<'a, I, A> + ParserMonad<'a, I, A> + AndThenParser<'a, I, A> {
  /// Sequential parser (discard first parser result) - implemented using and_then + map
  fn skip_left<P2, B>(self, p2: P2) -> Parser<'a, I, B, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a>
  where
    Self: 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: ParserRunner<'a, I, B> + 'a, {
    self.and_then(p2).map(move |(_, b)| b)
  }

  /// Sequential parser (discard second parser result) - implemented using and_then + map
  fn skip_right<B, P2>(self, p2: P2) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    Self: 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: ParserRunner<'a, I, B> + 'a, {
    self.and_then(p2).map(move |(a, _)| a)
  }
}

impl<'a, I: 'a, F, G, A, B> Mul<Parser<'a, I, B, G>> for Parser<'a, I, A, F>
where
    Self: SkipParser<'a, I, A>,
    A: Clone + 'a,
    B: Clone + 'a,
    Parser<'a, I, B, G>: ParserRunner<'a, I, B> + 'a,
    F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
    G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a,
{
  type Output = impl ParserRunner<'a, I, B>;

  fn mul(self, rhs: Parser<'a, I, B, G>) -> Self::Output {
    self.skip_left(rhs)
  }
}

impl<'a, I: 'a, F, G, A, B> Sub<Parser<'a, I, B, G>> for Parser<'a, I, A, F>
where
    Self: SkipParser<'a, I, A>,
    A: Clone + 'a,
    B: Clone + 'a,
    Parser<'a, I, B, G>: ParserRunner<'a, I, B> + 'a,
    F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
    G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a,
{
  type Output = impl ParserRunner<'a, I, A>;

  fn sub(self, rhs: Parser<'a, I, B, G>) -> Self::Output {
    self.skip_right(rhs)
  }
}

/// Implement SkipParser for all types that implement Parser, ParserMonad, and AndThenParser
impl<'a, T, I: 'a, A> SkipParser<'a, I, A> for T where
  T: ParserRunner<'a, I, A> + ParserMonad<'a, I, A> + AndThenParser<'a, I, A> + 'a
{
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::combinators::{elm_ref, tag};

  #[test]
  fn test_skip_left() {
    let text = "(abc";
    let input = text.chars().collect::<Vec<_>>();

    // '('をパースした後、"abc"をパースし、"abc"の結果を返す
    let p1 = elm_ref('(');
    let p2 = tag("abc");

    let parser = p1.skip_left(p2);

    let result = parser.parse(&input);

    assert!(result.is_success());
    assert_eq!(result.success().unwrap(), "abc");
  }

  #[test]
  fn test_skip_right() {
    let text = "abc)";
    let input = text.chars().collect::<Vec<_>>();

    // "abc"をパースした後、')'をパースし、"abc"の結果を返す
    let p1 = tag("abc");
    let p2 = elm_ref(')');

    let parser = p1.skip_right(p2);

    let result = parser.parse(&input);
    assert!(result.is_success());
    assert_eq!(result.success().unwrap(), "abc");
  }

  #[test]
  fn test_surround_manually() {
    let text = "(abc)";
    let input = text.chars().collect::<Vec<_>>();

    let left = elm_ref('(');
    let middle = tag("abc");
    let right = elm_ref(')');

    // surroundをskip_leftとskip_rightで手動で実装
    let parser = left.skip_left(middle.skip_right(right));

    let result = parser.parse(&input);

    assert!(result.is_success());
    assert_eq!(result.success().unwrap(), "abc");
  }
}
