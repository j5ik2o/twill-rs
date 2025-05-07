use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, ParserRunner};

/// Trait that adds optional parsing capability to all parsers
pub trait OptParser<'a, I: 'a, A>: ParserRunner<'a, I, A>
where
    Self: 'a, {
    /// Make a parser optional, wrapping successful results in Some and returning None on failure
    /// 
    /// This method creates a parser that:
    /// 1. Attempts to run the original parser
    /// 2. If successful, wraps the result in Some(value)
    /// 3. If unsuccessful, returns None without consuming input
    /// 
    /// The resulting parser never fails - it either returns Some(value) or None.
    fn opt(self) -> Parser<'a, I, Option<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Option<A>> + 'a>
    where
        A: Clone + 'a, { 
        // Conceptual implementation: or(map(attempt(parser), Some), successful(None))
        // Direct implementation with Parser::new instead of function composition
        // to avoid requiring Clone trait bounds
        Parser::new(move |parse_context: ParseContext<'a, I>| {
            // First try to run the parser with attempt (uncommitted)
            let result = self.clone().run(parse_context.with_same_state()).with_uncommitted();
            
            match result {
                // If successful, wrap the value in Some
                ParseResult::Success { parse_context, value, length } => {
                    ParseResult::successful(parse_context, Some(value), length)
                },
                // If unsuccessful, return None without failing
                ParseResult::Failure { parse_context, .. } => {
                    ParseResult::successful(parse_context, None, 0)
                }
            }
        })
    }
}

/// Add OptParser methods to all parsers
impl<'a, T, I: 'a, A> OptParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_opt_success() {
        // Test with a parser that succeeds
        let text: &str = "a";
        let input = text.chars().collect::<Vec<_>>();
        let parser = elm_ref('a').opt();

        let result = parser.parse(&input).to_result();
        println!("{:?}", result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(&'a'));
    }

    #[test]
    fn test_opt_failure() {
        // Test with a parser that fails
        let text: &str = "b";
        let input = text.chars().collect::<Vec<_>>();
        let parser = elm_ref('a').opt();

        let result = parser.parse(&input).to_result();
        println!("{:?}", result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_opt_with_sequential_parsers() {
        // Test opt in a sequence of parsers
        let text: &str = "ac";
        let input = text.chars().collect::<Vec<_>>();
        
        // Parse 'a', then optionally 'b', then 'c'
        let parser = elm_ref('a')
            .and_then(elm_ref('b').opt())
            .and_then(elm_ref('c'));

        let result = parser.parse(&input).to_result();
        println!("{:?}", result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ((&'a', None), &'c'));
    }

    #[test]
    fn test_multiple_opts() {
        // Test nested opt parsers
        let text: &str = "a";
        let input = text.chars().collect::<Vec<_>>();
        
        // Look for optional 'a' followed by optional 'b'
        let parser = elm_ref('a').opt().and_then(elm_ref('b').opt());

        let result = parser.parse(&input).to_result();
        println!("{:?}", result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (Some(&'a'), None));
    }
}
