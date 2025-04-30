use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::{FuncParser, Parser};
use crate::core::{CommittedStatus, ParseError};
use std::fmt::Display;

pub fn end<'a, I: 'a>() -> impl Parser<'a, I, ()>
where
  I: Display, {
  FuncParser::new(move |parse_context: ParseContext<'a, I>| {
    let input = parse_context.input();
    if let Some(actual) = input.get(0) {
      let msg = format!("expect end of input, found: {}", actual);
      let pc = parse_context.next();
      let pe = ParseError::of_mismatch(pc, 1, msg);
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

pub fn unit<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  successful((), 0)
}

pub fn lazy<'a, I: 'a, A, P, F>(f: F) -> impl Parser<'a, I, A>
where
  P: Parser<'a, I, A> + 'a,
  A: 'a,
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
