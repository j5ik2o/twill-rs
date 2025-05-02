use crate::core::{ClonableParser, ParseContext, ParseResult, Parser};
use std::marker::PhantomData;

pub struct FnParser<'a, I: 'a, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, {
  parser_fn: F,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A, F> FnParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  pub(crate) fn new(parser_fn: F) -> Self {
    Self {
      parser_fn,
      _phantom: PhantomData,
    }
  }
}

impl<'a, I, A, F> Parser<'a, I, A> for FnParser<'a, I, A, F>
where
  A: Clone + 'a,
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
{
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    (self.parser_fn)(parse_context)
  }
}

impl<'a, I, A, F> Clone for FnParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
{
  fn clone(&self) -> Self {
    FnParser {
      parser_fn: self.parser_fn.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<'a, I: 'a, A: Clone + 'a, F> ClonableParser<'a, I, A> for FnParser<'a, I, A, F> where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a
{
}
