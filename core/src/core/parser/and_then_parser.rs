use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::Parser;

/// Trait providing sequence-related parser operations
pub trait AndThenParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + Sized
where
  Self: 'a, {
  /// Sequential parser that uses a function to create the second parser
  fn and_then_with<F, P2, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    Self: 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a,
    F: Fn(A) -> P2 + Clone + 'a, {
    self.flat_map(f)
  }

  /// Sequential parser (conjunction) - implemented using flat_map and map (no Clone required)
  fn and_then<P2, B>(self, p2: P2) -> impl Parser<'a, I, (A, B)>
  where
    Self: 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a, {
    self.and_then_with(move |a| p2.clone().map(move |b| (a.clone(), b.clone())))
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> AndThenParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
