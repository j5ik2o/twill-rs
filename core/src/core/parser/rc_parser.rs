use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::RefCell;

use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// A wrapper that makes any parser cloneable using reference counting
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
  /// Create a new RcParser by wrapping a parser function
  pub fn new(f: F) -> Self {
    Self {
      parser_fn: Rc::new(f),
      _phantom: PhantomData,
    }
  }
}

/// Create a reusable RcParser from a parser factory function
///
/// This allows creating a parser that can be cloned and used multiple times,
/// even though the underlying parser might not implement Clone.
/// Each time the parser is run, the factory function is called to create a new parser instance.
pub fn reusable_parser<'a, I: 'a, A, P, F>(
  factory: F,
) -> RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
where
  F: Fn() -> P + 'a,
  P: Parser<'a, I, A>, {
  
  // factoryをRcでラップして共有
  let factory_rc = Rc::new(factory);
  
  RcParser::new(move |ctx| {
    // 毎回ファクトリを使って新しいパーサーインスタンスを生成
    let parser = factory_rc();
    parser.run(ctx)
  })
}

/// Create a reusable RcParser from a cloneable parser
///
/// This creates a parser that can be used multiple times by cloning the original.
pub fn reusable_with_clone<'a, I: 'a, A, P>(
  parser: P,
) -> RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
where
  P: Parser<'a, I, A> + Clone + 'a, {
  
  // クローン可能なパーサーを利用
  let parser_clone = parser;
  
  // reusable_parserを利用して実装
  reusable_parser(move || parser_clone.clone())
}

/// Create an optional reusable RcParser from an optional parser factory function
pub fn reusable_parser_opt<'a, I: 'a, A: 'a, P, F>(
  factory_opt: Option<F>,
) -> Option<RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>>
where
  F: Fn() -> P + 'a,
  P: Parser<'a, I, A>, {
  match factory_opt {
    Some(f) => Some(reusable_parser(f)),
    None => None,
  }
}

/// Create an optional reusable RcParser from an optional cloneable parser
pub fn reusable_with_clone_opt<'a, I: 'a, A, P>(
  parser_opt: Option<P>,
) -> Option<RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>>
where
  P: Parser<'a, I, A> + Clone + 'a, {
  parser_opt.map(reusable_with_clone)
}

impl<'a, I: 'a, A, F> Parser<'a, I, A> for RcParser<'a, I, A, F>
where
    A: 'a,
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
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
