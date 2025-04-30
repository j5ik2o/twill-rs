mod elements;

pub use elements::*;

use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::{FuncParser, Parser};
use crate::core::{CommittedStatus, ParseError};
use std::fmt::Display;

pub fn end<'a, I: 'a>() -> impl Parser<'a, I, ()>
where
  I: Display, {
  FuncParser::new(move |mut parse_context: ParseContext<'a, I>| {
    let input = parse_context.input();
    if let Some(actual) = input.get(0) {
      let msg = format!("expect end of input, found: {}", actual);
      parse_context.next_mut();
      let pe = ParseError::of_mismatch(parse_context, 1, msg);
      ParseResult::failed_with_uncommitted(pe)
    } else {
      ParseResult::successful(parse_context, (), 0)
    }
  })
}

/// Always successful parser
pub fn successful<'a, I: 'a, A: Clone + 'a>(value: A, length: usize) -> impl Parser<'a, I, A> {
  FuncParser::new(move |parse_context: ParseContext<'a, I>| ParseResult::successful(parse_context, value, length))
}

/// Returns a [Parser] that does nothing.<br/>
/// 何もしない[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use crate::twill_core::core::parser::combinators::unit;
/// use crate::twill_core::core::Parser;
/// use crate::twill_core::core::ParseResult;
///
/// let text: &str = "a";
///
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = unit();
///
/// let result: ParseResult<char, ()> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), ());
/// ```
pub fn unit<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  successful((), 0)
}

pub fn lazy<'a, I: 'a, A, P, F>(f: F) -> impl Parser<'a, I, A>
where
  A: 'a,
  P: Parser<'a, I, A> + 'a,
  F: FnOnce() -> P + 'a, {
  unit().flat_map(move |_| f())
}

pub fn failed<'a, I: 'a, A>(
  parse_error: ParseError<'a, I>,
  committed_status: CommittedStatus,
) -> impl Parser<'a, I, A> {
  FuncParser::new(move |_| ParseResult::failed(parse_error, committed_status))
}

/// Do nothing parser - does not consume input and returns no value
pub fn empty<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  FuncParser::new(move |parse_context: ParseContext<'a, I>| ParseResult::successful(parse_context, (), 0))
}
