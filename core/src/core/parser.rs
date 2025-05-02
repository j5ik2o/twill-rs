use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use std::marker::PhantomData;

mod and_then_parser;
mod attempt_parser;
mod binary_operator_parser;
mod collect_parser;
mod combinators;
mod conversion_parser;
mod logging_parser;
mod offset_parser;
mod operator_parser;
mod or_parser;
mod parser_monad;
mod rc_parser;
mod repeat_parser;
mod skip_parser;
mod transform_parser;

pub use and_then_parser::*;
pub use attempt_parser::*;
pub use binary_operator_parser::*;
pub use collect_parser::*;
pub use combinators::*;
pub use conversion_parser::*;
pub use operator_parser::*;
pub use or_parser::*;
pub use parser_monad::*;
pub use rc_parser::*;
pub use repeat_parser::*;
pub use skip_parser::*;
pub use transform_parser::*;

pub trait Parser<'a, I: 'a, A>: Sized + 'a {
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;

  fn parse(self, input: &'a [I]) -> ParseResult<'a, I, A>
  where
    Self: Sized, {
    let parse_context = ParseContext::new(input, 0);
    self.run(parse_context)
  }
}

/// Basic parser trait
pub trait ClonableParser<'a, I: 'a, A>: Parser<'a, I, A> + Clone + Sized + 'a {}

pub(crate) struct FnOnceParser<'a, I: 'a, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, {
  parser_fn: F,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A, F> FnOnceParser<'a, I, A, F>
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  pub(crate) fn new(parser_fn: F) -> Self {
    Self {
      parser_fn,
      _phantom: PhantomData,
    }
  }
}

impl<'a, I, A, F> Parser<'a, I, A> for FnOnceParser<'a, I, A, F>
where
    A: Clone + 'a,
    F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
{
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    (self.parser_fn)(parse_context)
  }
}

// ---

pub(crate) struct FnParser<'a, I: 'a, A, F>
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
