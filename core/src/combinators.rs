mod elements_combinators;
mod offset_combinators;
mod skip_combinators;
mod take_combinators;

pub use elements_combinators::*;
pub use offset_combinators::*;
pub use skip_combinators::*;
pub use take_combinators::*;

use crate::prelude::*;
use std::fmt::{Debug, Display};

pub fn successful<'a, I: 'a, A: Clone + 'a>(
  value: A,
) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a> {
  Parser::new(move |parse_context| ParseResult::successful(parse_context, value.clone(), 0))
}

pub fn failed<'a, I: Clone + 'a, A: 'a>(
  error: ParseError<'a, I>,
  committed_status: CommittedStatus,
) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a> {
  Parser::new(move |parse_context| ParseResult::failed(parse_context, error.clone(), committed_status))
}

pub fn end<'a, I: 'a>() -> Parser<'a, I, (), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, ()> + 'a>
where
  I: Display + Debug, {
  Parser::new(move |parse_context: ParseContext<'a, I>| {
    let input = parse_context.input();
    if let Some(actual) = input.first() {
      let msg = format!("expect end of input, found: {}", actual);
      let pc = parse_context.add_offset(1);
      let input = parse_context.input();
      let pe = ParseError::of_mismatch(input, pc.next_offset(), 1, msg);
      ParseResult::failed_with_uncommitted(parse_context, pe)
    } else {
      ParseResult::successful(parse_context, (), 0)
    }
  })
}

pub fn unit<'a, I: 'a>() -> Parser<'a, I, (), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, ()> + 'a> {
  successful(())
}

pub fn lazy<'a, I: 'a, A, P, F>(f: F) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
where
  A: 'a,
  P: ParserRunner<'a, I, A> + 'a,
  F: Fn() -> P + 'a, {
  Parser::new(move |pc| {
    let parser = f();
    parser.run(pc)
  })
}
