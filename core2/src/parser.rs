use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use std::marker::PhantomData;
use std::rc::Rc;

mod and_then_parser;
mod attempt_parser;
mod collect_parser;
pub mod parser_monad;
mod skip_parser;
mod transform_parser;
mod or_parser;

pub use and_then_parser::*;
pub use attempt_parser::*;
pub use collect_parser::*;
pub use parser_monad::*;
pub use skip_parser::*;
pub use transform_parser::*;

// Add Sized constraint here, as Parser methods take &self, but Monad methods will take self
pub trait Parser<'a, I: 'a, A>: Sized + 'a {
  fn run(&self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;

  fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A> {
    let parse_context = ParseContext::new(input, 0);
    self.run(parse_context)
  }
}

pub fn successful<'a, I: 'a, A: Clone + 'a>(value: A) -> impl Parser<'a, I, A> {
  RcParser::new(move |parse_context| ParseResult::successful(parse_context, value.clone(), 0))
}

// --- RcParser (Try without changes first) ---

pub struct RcParser<'a, I: 'a, A, F>
where
  // F constraint might need to change to FnMut or FnOnce (but Rc<FnOnce> is tricky)
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

// RcParser's Clone impl doesn't require F: Clone
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
