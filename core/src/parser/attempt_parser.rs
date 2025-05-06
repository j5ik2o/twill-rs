use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, ParserRunner};

pub trait AttemptParser<'a, I: 'a, A>: ParserRunner<'a, I, A>
where
  Self: 'a, {
  fn attempt(self) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    A: 'a, {
    Parser::new(move |parse_context: ParseContext<'a, I>| self.run(parse_context).with_uncommitted())
  }
}

impl<'a, T, I: 'a, A> AttemptParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}
