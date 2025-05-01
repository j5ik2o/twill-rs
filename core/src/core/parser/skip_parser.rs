use crate::core::parser::collect_parser::CollectParser;
use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::{FuncParser, Parser};
use crate::core::{ParseContext, ParseResult};
use std::ops::{Mul, Sub};

/// Trait providing sequence-related parser operations
pub trait SkipParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + Sized
where
  Self: 'a, {
  /// Sequential parser with lazy evaluation (discard first parser result) - implemented using flat_map
  fn skip_left_with<F, P2, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    Self: 'a,
    F: Fn() -> P2 + Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a, {
    self.flat_map(move |_| f())
  }

  /// Sequential parser (discard first parser result) - implemented using skip_left_with
  /// alias: p1 * p2 = p1.skip_left(p2)
  fn skip_left<P2, B>(self, p2: P2) -> impl Parser<'a, I, B>
  where
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a, {
    self.skip_left_with(move || p2.clone())
  }

  /// Sequential parser with lazy evaluation (discard second parser result) - implemented using flat_map
  fn skip_right_with<F, B, P2>(self, f: F) -> impl Parser<'a, I, A>
  where
    F: Fn() -> P2 + Clone + 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a, {
    self.flat_map(move |a| f().map(move |_| a.clone()))
  }

  /// Sequential parser (discard second parser result) - implemented using skip_right_with
  /// alias: p1 - p2 = p1.skip_right(p2)
  fn skip_right<B, P2>(self, p2: P2) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a, {
    self.skip_right_with(move || p2.clone())
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> SkipParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}

// alias: p1 * p2 = p1.skip_left(p2)
impl<'a, I: 'a, F, G, A, B> Mul<FuncParser<'a, I, B, G>> for FuncParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
  G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + Clone + 'a,
  A: Clone + 'a,
  B: Clone + 'a,
{
  type Output = impl Parser<'a, I, B>;

  fn mul(self, rhs: FuncParser<'a, I, B, G>) -> Self::Output {
    self.skip_left(rhs)
  }
}

// alias: p1 - p2 = p1.skip_right(p2)
impl<'a, I: 'a, F, G, A, B> Sub<FuncParser<'a, I, B, G>> for FuncParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
  G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + Clone + 'a,
  A: Clone + 'a,
  B: Clone + 'a,
{
  type Output = impl Parser<'a, I, A>;

  fn sub(self, rhs: FuncParser<'a, I, B, G>) -> Self::Output {
    self.skip_right(rhs)
  }
}
