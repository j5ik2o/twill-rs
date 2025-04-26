use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// Always successful parser
pub fn pure<'a, I: 'a, A: Clone>(value: A) -> impl Parser<'a, I, A> {
  let value = value.clone();
  move |input: &ParseContext<'a, I>| {
    let context = input.with_same_state();
    ParseResult::successful(value.clone(), context)
  }
}

/// Do nothing parser - does not consume input and returns no value
pub fn empty<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  move |input: &ParseContext<'a, I>| {
    // 同じ状態の新しいコンテキストを作成
    let context = input.with_same_state();
    ParseResult::successful((), context)
  }
}
