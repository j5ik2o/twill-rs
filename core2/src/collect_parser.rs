use crate::parse_result::ParseResult;
use crate::parser::{Parser, RcParser};

pub trait CollectParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized
where
  Self: 'a, {
  fn collect(self) -> impl Parser<'a, I, &'a [I]>
  where
    A: 'a, {
    RcParser::new(move |parse_context| match self.run(parse_context) {
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

impl<'a, T, I: 'a, A> CollectParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
