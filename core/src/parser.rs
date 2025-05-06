use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use std::marker::PhantomData;
use std::rc::Rc;

mod and_then_parser;
mod attempt_parser;
mod binary_operator_parser;
mod collect_parser;
mod conversion_parser;
mod logging_parser;
mod offset_parser;
mod or_parser;
mod parser_monad;
mod repeat_parser;
mod skip_parser;
mod transform_parser;

pub use and_then_parser::*;
pub use attempt_parser::*;
pub use binary_operator_parser::*;
pub use collect_parser::*;
pub use conversion_parser::*;
pub use logging_parser::*;
pub use offset_parser::*;
pub use or_parser::*;
pub use parser_monad::*;
pub use repeat_parser::*;
pub use skip_parser::*;
pub use transform_parser::*;

pub trait Parser<'a, I: 'a, A>: Clone + Sized + 'a {
  // type P<'p, XI: 'p, XA, XF> = RcParser<'p, XI, XA, XF> where XF: Fn(ParseContext<'p, XI>) -> ParseResult<'p, XI, XA> + 'p;

  fn run(&self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;

  fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A> {
    let parse_context = ParseContext::new(input, 0);
    self.run(parse_context)
  }
}

pub struct RcParser<'a, I: 'a, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a, {
  parser_fn: Rc<F>,
  _phantom: PhantomData<(&'a I, A)>,
}

impl<'a, I: 'a, A, F> RcParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  pub fn new(f: F) -> Self {
    Self {
      parser_fn: Rc::new(f),
      _phantom: PhantomData,
    }
  }
}

impl<'a, I: 'a, A, F> Parser<'a, I, A> for RcParser<'a, I, A, F>
where
  A: 'a,
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  fn run(&self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    (self.parser_fn)(parse_context)
  }
}

impl<'a, I: 'a, A, F> Clone for RcParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  fn clone(&self) -> Self {
    Self {
      parser_fn: Rc::clone(&self.parser_fn),
      _phantom: PhantomData,
    }
  }
}
