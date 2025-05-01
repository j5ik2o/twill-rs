use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use std::marker::PhantomData;

pub mod and_then_parser;
pub mod attempt_parser;
pub mod binary_operator_parser;
pub mod collect_parser;
pub mod combinators;
mod logging_parser;
mod offset_parser;
pub mod operator_parser;
pub mod or_parser;
pub mod parser_monad;
pub mod rc_parser;
mod repeat_parser;
pub mod skip_parser;
mod transform_parser;

pub use repeat_parser::*;
pub use transform_parser::*;

/// Basic parser trait
pub trait Parser<'a, I: 'a, A> {
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;

  fn parse(self, input: &'a [I]) -> ParseResult<'a, I, A>
  where
    Self: Sized, {
    let parse_context = ParseContext::new(input, 0);
    self.run(parse_context)
  }
}

pub(crate) struct FuncParser<'a, I: 'a, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, {
  parser_fn: F,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A, F> FuncParser<'a, I, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  pub(crate) fn new(parser_fn: F) -> Self {
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
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    (self.parser_fn)(parse_context)
  }
}
