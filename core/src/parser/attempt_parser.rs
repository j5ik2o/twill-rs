use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, RcParser};

pub trait AttemptParser<'a, I: 'a, A>: Parser<'a, I, A>
where
  Self: 'a, {
  fn attempt(self) -> RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    A: 'a, {
    RcParser::new(move |parse_context: ParseContext<'a, I>| self.run(parse_context).with_uncommitted())
  }
}

impl<'a, T, I: 'a, A> AttemptParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
