use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use std::marker::PhantomData;

mod and_then_parser;
mod attempt_parser;
mod binary_operator_parser;
mod collect_parser;
mod combinators;
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
pub use operator_parser::*;
pub use or_parser::*;
pub use parser_monad::*;
pub use rc_parser::*;
pub use repeat_parser::*;
pub use skip_parser::*;
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
