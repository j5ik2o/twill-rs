use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::and_then_parser::AndThenParser;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{Parser, RcParser};
use std::ops::{Mul, Sub};

/// Trait providing sequence-related parser operations
pub trait SkipParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + Sized
where
  Self: 'a, {
  /// Sequential parser (discard first parser result) - implemented using skip_left_with
  /// alias: p1 * p2 = p1.skip_left(p2)
  fn skip_left<P2, B>(&'a self, p2: &'a P2) -> impl Parser<'a, I, B>
  where
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + Clone + 'a, {
    self.and_then(p2).map(move|(_, b)| b.clone())
  }

  /// Sequential parser (discard second parser result) - implemented using skip_right_with
  /// alias: p1 - p2 = p1.skip_right(p2)
  fn skip_right<B, P2>(&'a self, p2: &'a P2) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + Clone + 'a, {
    self.and_then(p2).map(move|(a, _)| a.clone())
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> SkipParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}

// alias: p1 * p2 = p1.skip_left(p2)
// impl<'a, I: 'a, F, G, A, B> Mul<&'a RcParser<'a, I, B, G>> for RcParser<'a, I, A, F>
// where
//   F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
//   G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + Clone + 'a,
//   A: Clone + 'a,
//   B: Clone + 'a,
// {
//   type Output = impl Parser<'a, I, B>;
//
//   fn mul(self, rhs: &'a RcParser<'a, I, B, G>) -> Self::Output {
//     self.skip_left(rhs)
//   }
// }

// alias: p1 - p2 = p1.skip_right(p2)
// impl<'a, I: 'a, F, G, A, B> Sub<&'a RcParser<'a, I, B, G>> for RcParser<'a, I, A, F>
// where
//   F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
//   G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + Clone + 'a,
//   A: Clone + 'a,
//   B: Clone + 'a,
// {
//   type Output = impl Parser<'a, I, A>;
//
//   fn sub(self, rhs: &'a RcParser<'a, I, B, G>) -> Self::Output {
//     self.skip_right(rhs)
//   }
// }

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
