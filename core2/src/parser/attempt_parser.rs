use crate::parse_context::ParseContext;
use crate::parser::{Parser, RcParser};

pub trait AttemptParser<'a, I: 'a, A>: Parser<'a, I, A>
where
  Self: 'a, {
  fn attempt(self) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a, {
    RcParser::new(move |parse_context: ParseContext<'a, I>| self.run(parse_context).with_uncommitted())
  }
}

impl<'a, T, I: 'a, A> AttemptParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
