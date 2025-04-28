use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;

pub mod binary_operator_parser;
pub mod choice_parser;
pub mod operator_parser;
pub mod rc_parser;
pub mod sequence_parser;
pub mod transform_parser;

/// Basic parser trait
pub trait Parser<'a, I: 'a, A> {
  fn parse(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;
}

/// Treat closures as parsers
impl<'a, F, I, A> Parser<'a, I, A> for F
where
  F: FnOnce(ParseContext<'a, I>) -> ParseResult<'a, I, A>,
  I: 'a,
{
  fn parse(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A> {
    self(parse_context)
  }
}
