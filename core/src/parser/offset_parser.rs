use crate::prelude::*;

pub trait OffsetParser<'a, I: 'a, A>: ParserRunner<'a, I, A> + Sized
where
  Self: 'a, {
  fn last_offset(self) -> Parser<'a, I, usize, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, usize> + 'a> {
    Parser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        mut parse_context,
        length,
        ..
      } => {
        let pc = parse_context.advance(length);
        let last_offset = pc.last_offset().unwrap_or(0);
        ParseResult::successful(parse_context, last_offset, length)
      }
      ParseResult::Failure {
        parse_context,
        error,
        committed_status: is_committed,
      } => ParseResult::failed(parse_context, error, is_committed),
    })
  }

  fn offset(self) -> Parser<'a, I, usize, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, usize> + 'a>  {
    Parser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        mut parse_context,
        length,
        ..
      } => {
        let pc = parse_context.advance(length);
        let offset = parse_context.offset();
        ParseResult::successful(parse_context, pc.offset(), length)
      }
      ParseResult::Failure {
        parse_context,
        error,
        committed_status: is_committed,
      } => ParseResult::failed(parse_context, error, is_committed),
    })
  }
}

impl<'a, T, I: 'a, A> OffsetParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}
