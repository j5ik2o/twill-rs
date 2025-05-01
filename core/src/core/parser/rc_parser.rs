use std::marker::PhantomData;
use std::rc::Rc;

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

/// Convert any parser to an RcParser without requiring Clone
///
/// This creates a reusable parser that can be cloned and used multiple times.
/// The original parser is consumed only once on the first use.
pub fn to_rc_parser<'a, I: 'a, A, T>(
  parser: T,
) -> RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
where
  T: Parser<'a, I, A> + 'a, {
  use crate::core::committed_status::CommittedStatus;
  use crate::core::parse_error::ParseError;
  use std::cell::RefCell;

  // パーサーをOptionでラップして一度だけ使用可能にする
  let parser_cell = Rc::new(RefCell::new(Some(parser)));

  RcParser::new(move |ctx| {
    // borrow_mutを使用して可変参照を取得し、takeでOptionからパーサーを取り出す
    let parser_opt = parser_cell.borrow_mut().take();
    match parser_opt {
      Some(p) => p.run(ctx),
      None => {
        // パーサーが既に消費されている場合はエラーを返す
        let error = ParseError::of_custom(
          ctx.with_same_state(),
          None,
          "Parser has already been consumed.".to_string(),
        );
        ParseResult::failed(error, CommittedStatus::Uncommitted)
      }
    }
  })
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

/// Convert an optional parser to an optional RcParser without requiring Clone
pub fn to_rc_parser_opt<'a, I: 'a, A: 'a, T>(
  parser_opt: Option<T>,
) -> Option<RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>>
where
  T: Parser<'a, I, A> + 'a, {
  parser_opt.map(to_rc_parser)
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

impl<'a, I: 'a, A, F> Parser<'a, I, A> for RcParser<'a, I, A, F>
where
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
