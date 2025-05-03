use crate::parser::{successful, Parser};
use crate::parser_monad::ParserMonad;

/// Trait providing sequence-related parser operations (consuming self)
pub trait AndThenParser<'a, I: 'a, A>: ParserMonad<'a, I, A> {
  // Removed Sized here, it's on Parser now
  /// Sequential parser (conjunction) - implemented using flat_map and successful (consuming self)
  fn and_then<P2, B>(self, p2: P2) -> impl Parser<'a, I, (A, B)>
  where
    Self: 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + Clone + 'a, {
    self.flat_map(move |a| {
      let a_clone = a.clone();
      p2.clone().flat_map(move |b| successful((a_clone.clone(), b)))
    })
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> AndThenParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
