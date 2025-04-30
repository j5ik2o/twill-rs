use crate::core::parser::Parser;
use crate::core::parser_monad::ParserMonad;

/// Trait providing sequence-related parser operations
pub trait SkipParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + Sized
where
    Self: 'a, {
    /// Sequential parser with lazy evaluation (discard first parser result) - implemented using flat_map
    fn skip_left_with<F, P2, B>(self, f: F) -> impl Parser<'a, I, B>
    where
        F: FnOnce() -> P2,
        P2: Parser<'a, I, B>, {
        self.flat_map(move |_| f())
    }

    /// Sequential parser (discard first parser result) - implemented using skip_left_with
    /// alias: p1 * p2 = p1.skip_left(p2)
    fn skip_left<P2, B>(self, p2: P2) -> impl Parser<'a, I, B>
    where
        P2: Parser<'a, I, B>, {
        self.skip_left_with(move || p2)
    }

    /// Sequential parser with lazy evaluation (discard second parser result) - implemented using flat_map
    fn skip_right_with<F, P2>(self, f: F) -> impl Parser<'a, I, A>
    where
        F: FnOnce() -> P2,
        P2: Parser<'a, I, ()>, {
        self.flat_map(move |a| f().map(move |_| a))
    }

    /// Sequential parser (discard second parser result) - implemented using skip_right_with
    /// alias: p1 - p2 = p1.skip_right(p2)
    fn skip_right<P2>(self, p2: P2) -> impl Parser<'a, I, A>
    where
        P2: Parser<'a, I, ()>, {
        self.skip_right_with(move || p2)
    }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> SkipParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
