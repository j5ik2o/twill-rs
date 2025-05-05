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
        parse_context, length, ..
      } => {
        let slice = parse_context.slice_with_len(length);
        ParseResult::Success {
          parse_context,
          value: slice,
          length,
        }
      }
      ParseResult::Failure {
        parse_context,
        error,
        committed_status,
      } => ParseResult::failed(parse_context, error, committed_status),
    })
  }
}

impl<'a, T, I: 'a, A> CollectParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  fn test_collect() {
    let text: &str = "a";
    let input = text.chars().collect::<Vec<_>>();
    let p = elm_ref('a').collect();

    let result = p.parse(&input).to_result();
    println!("{:?}", result);

    assert!(result.is_ok());
  }
}
