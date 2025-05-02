mod elements_combinators;
mod offset_combinators;
mod take_combinators;

pub use elements_combinators::*;
pub use offset_combinators::*;
pub use take_combinators::*;

use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::rc_parser::reusable_parser;
use crate::core::parser::{ClonableParser, FnParser};
use crate::core::{CommittedStatus, FnOnceParser, ParseError, Parser, SkipParser};
use std::fmt::Display;

pub fn end<'a, I: 'a>() -> impl Parser<'a, I, ()>
where
  I: Display, {
  FnOnceParser::new(move |mut parse_context: ParseContext<'a, I>| {
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
pub fn successful<'a, I: 'a, A: Clone + 'a>(value: A) -> impl ClonableParser<'a, I, A> {
  let value_clone = value;
  reusable_parser(move || {
    let v = value_clone.clone();
    FnParser::new(move |parse_context: ParseContext<'a, I>| ParseResult::successful(parse_context, v.clone(), 0))
  })
}

/// Returns a [ClonableParser] that does nothing.<br/>
/// 何もしない[ClonableParser]を返します。
///
/// # Example
///
/// ```rust
/// use twill_core::prelude::*;
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
pub fn unit<'a, I: 'a>() -> impl ClonableParser<'a, I, ()> {
  successful(())
}

pub fn lazy<'a, I: 'a, A, P, F>(f: F) -> impl ClonableParser<'a, I, A>
where
  A: Clone + 'a,
  P: ClonableParser<'a, I, A> + 'a,
  F: Fn() -> P + Clone + 'a, {
  reusable_parser(f)
}

pub fn failed<'a, I: Clone + 'a, A: Clone + 'a>(
  parse_error: ParseError<'a, I>,
  committed_status: CommittedStatus,
) -> impl ClonableParser<'a, I, A> {
  let err_clone = parse_error.clone();
  let status = committed_status;
  reusable_parser(move || {
    let err = err_clone.clone();
    FnParser::new(move |_| ParseResult::failed(err.clone(), status))
  })
}

/// Do nothing parser - does not consume input and returns no value
pub fn empty<'a, I: 'a>() -> impl ClonableParser<'a, I, ()> {
  reusable_parser(move || {
    FnParser::new(move |parse_context: ParseContext<'a, I>| ParseResult::successful(parse_context, (), 0))
  })
}

/// Return a [ClonableParser] that skips the previous and following [ClonableParser]s.
///
/// - lp: left side parser
/// - parser: central parser
/// - rp: right side parser
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "(abc)";
/// let input = text.chars().collect::<Vec<_>>();
///
/// // まず正しく左右の括弧をパースする
/// let left_parser = elm_ref('(');
/// let content_parser = tag("abc");
/// let right_parser = elm_ref(')');
///
/// let parser = surround(left_parser, content_parser, right_parser);
///
/// let result = parser.parse(&input);
///
/// println!("{:?}", result);
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn surround<'a, I: Clone + 'a, A, B, C, P1, P2, P3>(lp: P1, parser: P2, rp: P3) -> impl ClonableParser<'a, I, B>
where
  A: Clone + 'a,
  B: Clone + 'a,
  C: Clone + 'a,
  P1: ClonableParser<'a, I, A> + 'a,
  P2: ClonableParser<'a, I, B> + 'a,
  P3: ClonableParser<'a, I, C> + 'a, {
  lp.skip_left(parser).skip_right(rp)
}
