use crate::parser::parser_monad::ParserMonad;
use crate::parser::Parser;

/// Trait providing sequence-related parser operations (consuming self)
pub trait AndThenParser<'a, I: 'a, A>: ParserMonad<'a, I, A> {
  /// Sequential parser (conjunction) - implemented directly using RcParser (consuming self)
  fn and_then<P2, B>(self, p2: P2) -> impl Parser<'a, I, (A, B)>
  where
    Self: Clone + 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + Clone + 'a, {
    self.clone().flat_map(move |a| p2.clone().map(move |b| (a.clone(), b)))
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> AndThenParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  fn test_and_then() {
    let text: &str = "ab";
    let input = text.chars().collect::<Vec<_>>();
    let p = elm_ref('a').and_then(elm_ref('b'));

    let result = p.parse(&input).to_result();
    println!("{:?}", result);

    assert!(result.is_ok());
    let (a, b) = result.unwrap();
    assert_eq!(*a, 'a');
    assert_eq!(*b, 'b');
  }
}
