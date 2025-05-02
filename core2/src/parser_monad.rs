use crate::parse_result::ParseResult;
use crate::parser::{successful, Parser, RcParser};

/// Trait providing parser transformation methods (consuming self)
pub trait ParserMonad<'a, I: 'a, A>: Parser<'a, I, A> {
    /// Transform success result (consuming self)
    fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
    where
        Self: 'a, // Sized moved to Parser trait
        A: 'a,
        B: Clone + 'a,
    // F is moved, 'a might be unnecessary? Clone is unnecessary.
        F: Fn(A) -> B + 'a; // Try with Fn first

    /// Chain parsers (consuming self)
    fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
    where
        Self: 'a, // Sized moved to Parser trait
        A: 'a,
        B: 'a,
        P: Parser<'a, I, B> + 'a,
    // F is moved, 'a might be unnecessary? Clone is unnecessary.
        F: Fn(A) -> P + 'a; // Try with Fn first
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserMonad<'a, I, A> for T
where T: Parser<'a, I, A> + 'a { // T: Sized moved to Parser trait

    // map implementation
    fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
    where
        Self: 'a,
        A: 'a,
        B: Clone + 'a,
        F: Fn(A) -> B + 'a, // Try with Fn
    {
        // Move self to call flat_map
        self.flat_map(move |a| successful(f(a))) // f is moved
    }

    // flat_map implementation
    fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
    where
        Self: 'a,
        A: 'a,
        B: 'a,
        P: Parser<'a, I, B> + 'a,
        F: Fn(A) -> P + 'a, // Try with Fn
    {
        // The closure passed to RcParser::new might become FnOnce
        // Let's write it directly first
        RcParser::new(move |parse_context| { // self and f are moved
            match self.run(parse_context) { // Use self directly
                ParseResult::Success {
                    parse_context,
                    value,
                    length,
                } => {
                    // f(value) returns P. f is moved.
                    f(value).run(parse_context.advance(length))
                }
                ParseResult::Failure {
                    error,
                    committed_status,
                } => ParseResult::failed(error, committed_status),
            }
        })
    }
}
