use crate::core::{ClonableParser, FnParser, ParseError, ParseResult};

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
pub fn skip<'a, I: Clone + 'a>(n: usize) -> impl ClonableParser<'a, I, ()> {
  FnParser::new(move |parse_context| {
    let input = parse_context.input();
    if input.len() >= n {
      ParseResult::successful(parse_context, (), n)
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context))
    }
  })
}
