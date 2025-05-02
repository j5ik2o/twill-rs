use crate::parser::{successful, Parser};
use crate::parser_monad::ParserMonad;

/// Trait providing sequence-related parser operations (consuming self)
pub trait AndThenParser<'a, I: 'a, A>: ParserMonad<'a, I, A> { // Removed Sized here, it's on Parser now
  /// Sequential parser (conjunction) - implemented using flat_map and successful (consuming self)
  fn and_then<P2, B>(self, p2: P2) -> impl Parser<'a, I, (A, B)> // Changed &'a self to self
  where
    Self: 'a, // Sized is implied by taking self
    A: Clone + 'a,
    B: Clone + 'a,
    // Remove temporary Copy constraint
    P2: Parser<'a, I, B> + Clone + 'a,
  {
      // p2 doesn't need cloning outside if it's moved into the closure?
      // Let's try moving p2 directly.
      self.flat_map(move |a| { // self is moved here
          let a_clone = a.clone();
          // p2 is moved into this closure.
          // The inner flat_map also takes self, so p2 is moved here.
          // Clone p2 before calling flat_map as the outer closure might be FnMut.
          p2.clone().flat_map(move |b| { // p2 is cloned here before moving
              // successful creates a new parser
              successful((a_clone.clone(), b))
          })
      })
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> AndThenParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
