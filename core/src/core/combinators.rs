use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::{FuncParser, Parser};
use crate::core::{CommittedStatus, ParseError};

/// Always successful parser
pub fn successful<'a, I: 'a, A: Clone + 'a>(value: A, length: usize) -> impl Parser<'a, I, A> {
  FuncParser::new(move |parse_context: ParseContext<'a, I>| ParseResult::successful(parse_context, value, length))
}

pub fn failed<'a, I: 'a, A>(
  parse_error: ParseError<'a, I>,
  committed_status: CommittedStatus,
) -> impl Parser<'a, I, A> {
  FuncParser::new(move |_| ParseResult::failed(parse_error, committed_status))
}

/// Do nothing parser - does not consume input and returns no value
pub fn empty<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  FuncParser::new(move |parse_context: ParseContext<'a, I>| {
    // 同じ状態の新しいコンテキストを作成
    ParseResult::successful(parse_context, (), 0)
  })
}
