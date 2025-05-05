mod elements_combinators;
mod offset_combinators;
mod skip_combinators;
mod take_combinators;

pub use elements_combinators::*;
pub use offset_combinators::*;
pub use skip_combinators::*;
pub use take_combinators::*;

use crate::prelude::*;
use std::fmt::Display;

#[inline(always)]
pub fn successful<'a, I: 'a, A: Clone + 'a>(value: A) -> impl Parser<'a, I, A> {
  RcParser::new(move |parse_context| ParseResult::successful(parse_context, value.clone(), 0))
}

#[inline(always)]
pub fn failed<'a, I: Clone + 'a, A: 'a>(
  error: ParseError<'a, I>,
  committed_status: CommittedStatus,
) -> impl Parser<'a, I, A> {
  RcParser::new(move |parse_context| ParseResult::failed(parse_context, error.clone(), committed_status))
}

pub fn end<'a, I: 'a>() -> impl Parser<'a, I, ()>
where
  I: Display, {
  RcParser::new(move |mut parse_context: ParseContext<'a, I>| {
    let input = parse_context.input();
    if let Some(actual) = input.first() {
      let msg = format!("expect end of input, found: {}", actual);
      parse_context.next_mut();
      let input = parse_context.input();
      let pe = ParseError::of_mismatch(input, 0, 1, msg);
      ParseResult::failed_with_uncommitted(parse_context, pe)
    } else {
      ParseResult::successful(parse_context, (), 0)
    }
  })
}

pub fn unit<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  successful(())
}

pub fn lazy<'a, I: 'a, A, P, F>(f: F) -> impl Parser<'a, I, A>
where
  A: 'a,
  P: Parser<'a, I, A> + 'a,
  F: Fn() -> P + 'a, {
  RcParser::new(move |pc| {
    let parser = f();
    parser.run(pc)
  })
}
