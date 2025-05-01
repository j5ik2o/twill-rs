mod elements_combinators;
mod take_combinators;

pub use elements_combinators::*;
pub use take_combinators::*;

use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::rc_parser::reusable_parser;
use crate::core::parser::{FuncParser, Parser};
use crate::core::{CommittedStatus, ParseError};
use std::fmt::Display;

pub fn end<'a, I: 'a>() -> impl Parser<'a, I, ()>
where
  I: Display, {
  reusable_parser(move || {
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
  })
}

/// Always successful parser
pub fn successful<'a, I: 'a, A: Clone + 'a>(value: A, length: usize) -> impl Parser<'a, I, A> {
  let value_clone = value;
  reusable_parser(move || {
    let v = value_clone.clone();
    FuncParser::new(move |parse_context: ParseContext<'a, I>| ParseResult::successful(parse_context, v.clone(), length))
  })
}

/// Returns a [Parser] that does nothing.<br/>
/// 何もしない[Parser]を返します。
///
/// # Example
///
/// ```rust
/// use crate::twill_core::core::parser::unit;
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
  A: Clone + 'a,
  P: Parser<'a, I, A> + 'a,
  F: Fn() -> P + Clone + 'a, {
  use crate::core::parser::rc_parser::reusable_parser;
  reusable_parser(f)
}

pub fn failed<'a, I: Clone + 'a, A: Clone + 'a>(
  parse_error: ParseError<'a, I>,
  committed_status: CommittedStatus,
) -> impl Parser<'a, I, A> {
  let err_clone = parse_error.clone();
  let status = committed_status;
  reusable_parser(move || {
    let err = err_clone.clone();
    FuncParser::new(move |_| ParseResult::failed(err.clone(), status))
  })
}

/// Do nothing parser - does not consume input and returns no value
pub fn empty<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  reusable_parser(move || {
    FuncParser::new(move |parse_context: ParseContext<'a, I>| ParseResult::successful(parse_context, (), 0))
  })
}
