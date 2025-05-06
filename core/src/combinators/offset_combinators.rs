use std::fmt::Debug;
use crate::prelude::*;

/// Returns a [ClonableParser] that skips the specified number of elements.
///
/// - size: a size of elements
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = skip(3).skip_left(tag("def"));
///
/// let result = parser.parse(&input);
///
/// println!("{:?}", result);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "def");
/// ```
pub fn skip<'a, I: Clone + 'a>(n: usize) -> impl ParserRunner<'a, I, ()> where I: Debug {
  Parser::new(move |parse_context| {
    let input = parse_context.input();
    if input.len() >= n {
      ParseResult::successful(parse_context, (), n)
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
    }
  })
}
