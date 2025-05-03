use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
// use crate::parser::and_then_parser::AndThenParser; // Directly used AndThenParser is not needed if using flat_map
use crate::parser::and_then_parser::AndThenParser;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{Parser, RcParser};
use std::ops::{Mul, Sub};

/// Trait providing sequence-related parser operations (consuming self)
pub trait SkipParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + AndThenParser<'a, I, A> // Add AndThenParser bound
where
  // Self: 'a, // No longer needed as self is consumed
{
  /// Sequential parser (discard first parser result) - implemented using and_then + map
  /// alias: p1 * p2 = p1.skip_left(p2)
  fn skip_left<P2, B>(self, p2: &'a P2) -> impl Parser<'a, I, B> // Changed to take self
  where
    A: 'a, // A is consumed by and_then
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a, { // p2 doesn't need Clone if only used in and_then
    // Now this should work because and_then and map take self
    self.and_then(p2).map(move |(_, b)| b.clone())
  }

  /// Sequential parser (discard second parser result) - implemented using and_then + map
  /// alias: p1 - p2 = p1.skip_right(p2)
  fn skip_right<B, P2>(self, p2: &'a P2) -> impl Parser<'a, I, A> // Changed to take self
  where
    A: Clone + 'a, // A needs Clone because it's returned by map
    B: 'a, // B is consumed by and_then
    P2: Parser<'a, I, B> + 'a, { // p2 doesn't need Clone if only used in and_then
    // Now this should work because and_then and map take self
    self.and_then(p2).map(move |(a, _)| a.clone())
  }
}

// No blanket impl needed - default methods in the trait should suffice.

// alias: p1 * p2 = p1.skip_left(p2)
impl<'a, I: 'a, F, G, A, B> Mul<&'a RcParser<'a, I, B, G>> for RcParser<'a, I, A, F>
where
  Self: SkipParser<'a, I, A>, // Self must implement SkipParser
  A: 'a,
  B: Clone + 'a,
  RcParser<'a, I, B, G>: Parser<'a, I, B> + 'a, // P2 for skip_left
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, // RcParser bound
  G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a, // RcParser bound
{
  // Define Output type directly using impl Trait
  type Output = impl Parser<'a, I, B>;

  fn mul(self, rhs: &'a RcParser<'a, I, B, G>) -> Self::Output {
    // skip_left takes self, matching fn mul(self, ...)
    self.skip_left(rhs)
  }
}

// alias: p1 - p2 = p1.skip_right(p2)
impl<'a, I: 'a, F, G, A, B> Sub<&'a RcParser<'a, I, B, G>> for RcParser<'a, I, A, F>
where
  Self: SkipParser<'a, I, A>, // Self must implement SkipParser
  A: Clone + 'a,
  B: 'a,
  RcParser<'a, I, B, G>: Parser<'a, I, B> + 'a, // P2 for skip_right
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, // RcParser bound
  G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a, // RcParser bound
{
  // Define Output type directly using impl Trait
  type Output = impl Parser<'a, I, A>;

  fn sub(self, rhs: &'a RcParser<'a, I, B, G>) -> Self::Output {
    // skip_right takes self, matching fn sub(self, ...)
    self.skip_right(rhs)
  }
}

// #[cfg(test)]
// mod tests {
//
//   #[test]
//   fn test_skip_left() {
//     let text = "(abc)";
//     let input = text.chars().collect::<Vec<_>>();
//
//     // '('をパースした後、"abc"をパースし、"abc"の結果を返す
//     let p1 = elm_ref('(');
//     let p2 = tag("abc");
//
//     let parser = p1.skip_left(p2);
//
//     let result = parser.parse(&input);
//
//     assert!(result.is_success());
//     assert_eq!(result.success().unwrap(), "abc");
//   }
//
//   #[test]
//   fn test_skip_right() {
//     let text = "abc)";
//     let input = text.chars().collect::<Vec<_>>();
//
//     // "abc"をパースした後、')'をパースし、"abc"の結果を返す
//     let p1 = tag("abc");
//     let p2 = elm_ref(')');
//
//     let parser = p1.skip_right(p2);
//
//     let result = parser.parse(&input);
//     assert!(result.is_success());
//     assert_eq!(result.success().unwrap(), "abc");
//   }
//
//   #[test]
//   fn test_surround_manually() {
//     let text = "(abc)";
//     let input = text.chars().collect::<Vec<_>>();
//
//     let left = elm_ref('(');
//     let middle = tag("abc");
//     let right = elm_ref(')');
//
//     // surroundをskip_leftとskip_rightで手動で実装
//     let parser = left.skip_left(middle).skip_right(right);
//
//     let result = parser.parse(&input);
//
//     assert!(result.is_success());
//     assert_eq!(result.success().unwrap(), "abc");
//   }
// }
