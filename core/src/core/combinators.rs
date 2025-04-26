use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// Always successful parser
pub fn pure<'a, I: 'a, A: Clone>(value: A) -> impl Parser<'a, I, A> {
  let value = value.clone();
  move |_input: &'a [I]| ParseResult::successful(value.clone(), 0)
}

/// Do nothing parser - does not consume input and returns no value
pub fn empty<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  move |_input: &'a [I]| ParseResult::successful((), 0)
}
