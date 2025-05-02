use crate::core::parser::FnParser;
use crate::core::{ClonableParser, ParseResult};

pub trait CollectParser<'a, I: 'a, A>: ClonableParser<'a, I, A> + Sized
where
  Self: 'a, {
  fn collect(self) -> impl ClonableParser<'a, I, &'a [I]>
  where
    A: 'a, {
    FnParser::new(move |parse_context| match self.clone().run(parse_context) {
      ParseResult::Success {
        parse_context: pc1,
        length,
        ..
      } => {
        let slice = pc1.slice_with_len(length);
        ParseResult::Success {
          parse_context: pc1,
          length,
          value: slice,
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    })
  }
}

impl<'a, T, I: 'a, A> CollectParser<'a, I, A> for T where T: ClonableParser<'a, I, A> + 'a {}
