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

/// Convert any parser to an RcParser
pub fn to_rc_parser<'a, I: 'a, A: 'a, T>(
  parser: T,
) -> RcParser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
where
  T: Parser<'a, I, A> + Clone + 'a, {
  // パーサーをクローンして保持
  struct ParserHolder<'a, I: 'a, A: 'a, T: Parser<'a, I, A> + Clone + 'a> {
    parser: T,
    _phantom: PhantomData<(&'a I, A)>,
  }

  impl<'a, I: 'a, A: 'a, T: Parser<'a, I, A> + Clone + 'a> Clone for ParserHolder<'a, I, A, T> {
    fn clone(&self) -> Self {
      Self {
        parser: self.parser.clone(),
        _phantom: PhantomData,
      }
    }
  }

  let holder = Rc::new(ParserHolder {
    parser,
    _phantom: PhantomData,
  });

  RcParser::new(move |ctx| {
    let parser = holder.parser.clone();
    parser.parse(ctx)
  })
}

/// Convert any parser to an RcParser without requiring Clone
///
/// This is useful for parsers that can only be used once.
/// The returned RcParser can be cloned and used multiple times,
/// but it will only successfully parse on the first use.
pub fn to_single_use_rc_parser<'a, I: 'a, A: 'a, T>(
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
      Some(p) => p.parse(ctx),
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

impl<'a, I: 'a, A, F> Parser<'a, I, A> for RcParser<'a, I, A, F>
where
  F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  fn parse(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
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
