use crate::core::parse_context::ParseContext;
use crate::core::parser::{ClonableParser, FnParser};

pub trait AttemptParser<'a, I: 'a, A>: ClonableParser<'a, I, A>
where
  Self: 'a, {
  fn attempt(self) -> impl ClonableParser<'a, I, A>
  where
    A: Clone + 'a, {
    FnParser::new(move |parse_context: ParseContext<'a, I>| self.clone().run(parse_context).with_uncommitted())
  }
}

impl<'a, T, I: 'a, A> AttemptParser<'a, I, A> for T where T: ClonableParser<'a, I, A> + 'a {}
