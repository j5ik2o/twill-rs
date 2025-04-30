use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use std::marker::PhantomData;

pub mod and_then_parser;
pub mod attempt_parser;
pub mod binary_operator_parser;
pub mod or_parser;
pub mod collect_parser;
pub mod operator_parser;
pub mod rc_parser;
pub mod repeat_parser;
pub mod skip_parser;
pub mod transform_parser;
pub mod parser_monad;

/// Basic parser trait
pub trait Parser<'a, I: 'a, A> {
  fn parse(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;
}

pub struct FuncParser<'a, I: 'a, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, {
  parser_fn: F,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A, F> FuncParser<'a, I, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  pub fn new(parser_fn: F) -> Self {
    FuncParser {
      parser_fn,
      _phantom: PhantomData,
    }
  }
}

impl<'a, I, A, F> Parser<'a, I, A> for FuncParser<'a, I, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  fn parse(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    (self.parser_fn)(parse_context)
  }
}
