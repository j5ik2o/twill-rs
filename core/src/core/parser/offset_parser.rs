use crate::core::parser::FnParser;
use crate::core::{ClonableParser, ParseResult};

pub trait OffsetParser<'a, I: 'a, A>: ClonableParser<'a, I, A> + Sized
where
  Self: 'a, {
  fn last_offset(self) -> impl ClonableParser<'a, I, usize> {
    FnParser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        mut parse_context,
        length,
        ..
      } => {
        parse_context.advance_mut(length);
        let last_offset = parse_context.last_offset().unwrap_or(0);
        ParseResult::successful(parse_context, last_offset, length)
      }
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }

  fn offset(self) -> impl ClonableParser<'a, I, usize> {
    FnParser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        mut parse_context,
        length,
        ..
      } => {
        parse_context.advance_mut(length);
        let offset = parse_context.offset();
        ParseResult::successful(parse_context, offset, length)
      }
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }
}

impl<'a, T, I: 'a, A> OffsetParser<'a, I, A> for T where T: ClonableParser<'a, I, A> + 'a {}
