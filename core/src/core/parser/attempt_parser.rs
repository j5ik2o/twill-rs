use crate::core::parse_context::ParseContext;
use crate::core::parser::{FuncParser, Parser};

pub trait AttemptParser<'a, I: 'a, A>: Parser<'a, I, A>
where
  Self: 'a, {
  fn attempt(self) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a, {
    FuncParser::new(move |parse_context: ParseContext<'a, I>| self.clone().run(parse_context).with_uncommitted())
  }
}

impl<'a, T, I: 'a, A> AttemptParser<'a, I, A> for T where T: Parser<'a, I, A> + Clone + 'a {}
