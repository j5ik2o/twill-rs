use crate::core::parse_result::ParseResult;

/// Basic parser trait
pub trait Parser<'a, I: 'a, A> {
  fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A>;
}

/// Treat closures as parsers
impl<'a, F, I, A> Parser<'a, I, A> for F
where
  F: Fn(&'a [I]) -> ParseResult<'a, I, A>,
  I: 'a,
{
  fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A> {
    self(input)
  }
}
