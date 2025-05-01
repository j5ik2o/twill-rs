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
pub trait Parser<'a, I: 'a, A>: Clone + Sized + 'a {
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;

  fn parse(self, input: &'a [I]) -> ParseResult<'a, I, A>
  where
    Self: Sized, {
    let parse_context = ParseContext::new(input, 0);
    self.run(parse_context)
  }
}

// Parserを参照として実装できるようにするためのヘルパートレイト
pub trait ParserRef<'a, I: 'a, A> {
  fn run_ref(&self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;
}

// Cloneを実装したParserに対して、参照からの実行を可能にする
impl<'a, P, I: 'a, A> ParserRef<'a, I, A> for P
where
  P: Parser<'a, I, A> + Clone,
{
  fn run_ref(&self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    self.clone().run(parse_context)
  }
}

pub(crate) struct FuncParser<'a, I: 'a, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, {
  parser_fn: F,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A, F> FuncParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
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
  A: Clone + 'a,
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
{
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    (self.parser_fn)(parse_context)
  }
}

impl<'a, I, A, F> Clone for FuncParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + Clone + 'a,
{
  fn clone(&self) -> Self {
    FuncParser {
      parser_fn: self.parser_fn.clone(),
      _phantom: PhantomData,
    }
  }
}
