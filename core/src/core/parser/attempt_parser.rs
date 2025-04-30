use crate::core::parser::FuncParser;
use crate::core::{ParseContext, Parser};

pub trait AttemptParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized
where
  Self: 'a, {
  /// Transforms any failure into an uncommitted failure
  /// This allows the parser to be used in an or_with operation even if it would normally commit
  fn attempt(self) -> impl Parser<'a, I, A> {
    FuncParser::new(move |parse_context: ParseContext<'a, I>| self.run(parse_context).with_uncommitted())
  }
}

impl<'a, T, I: 'a, A> AttemptParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
