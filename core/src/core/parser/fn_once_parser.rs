use crate::core::{ParseContext, ParseResult, Parser};
use std::marker::PhantomData;

pub struct FnOnceParser<'a, I: 'a, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, {
  parser_fn: F,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A, F> FnOnceParser<'a, I, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  pub fn new(parser_fn: F) -> Self {
    Self {
      parser_fn,
      _phantom: PhantomData,
    }
  }
}

impl<'a, I, A, F> Parser<'a, I, A> for FnOnceParser<'a, I, A, F>
where
  A: Clone + 'a,
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    (self.parser_fn)(parse_context)
  }
}
