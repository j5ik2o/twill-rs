use crate::parser::{Parser, SkipParser};

/// Return a [ClonableParser] that skips the previous and following [ClonableParser]s.
///
/// - lp: left side parser
/// - parser: central parser
/// - rp: right side parser
///
/// # Example
///
/// ```rust
/// # use twill_core2::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "(abc)";
/// let input = text.chars().collect::<Vec<_>>();
///
/// // まず正しく左右の括弧をパースする
/// let left_parser = elm_ref('(');
/// let content_parser = tag("abc");
/// let right_parser = elm_ref(')');
///
/// let parser = surround(left_parser, content_parser, right_parser);
///
/// let result = parser.parse(&input);
///
/// println!("{:?}", result);
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn surround<'a, I: Clone + 'a, A, B, C, P1, P2, P3>(lp: P1, parser: P2, rp: P3) -> impl Parser<'a, I, B>
where
  A: Clone + 'a,
  B: Clone + 'a,
  C: Clone + 'a,
  P1: Parser<'a, I, A> + Clone + 'a,
  P2: Parser<'a, I, B> + Clone + 'a,
  P3: Parser<'a, I, C> + Clone + 'a, {
  lp.skip_left(parser).skip_right(rp)
}
