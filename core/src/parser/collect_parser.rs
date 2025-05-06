use std::fmt::Debug;
use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{ParserRunner, Parser};

pub trait CollectParser<'a, I: 'a, A>: ParserRunner<'a, I, A> + Sized
where
  Self: 'a, {
  fn collect(self) -> Parser<'a, I, &'a [I], impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a [I]> + 'a>
  where
    I : Debug + 'a,
    A: Debug + 'a, {
    Parser::new(move |parse_context| {
      println!("start: collect");
      let r = match self.run(parse_context) {
        ParseResult::Success {
          parse_context, length, ..
        } => {
          println!("length: {}", length);
          println!("parse_context: {:?}", parse_context);
          let slice = parse_context.slice_with_len(length);
          ParseResult::successful(parse_context, slice, length)
        }
        ParseResult::Failure {
          parse_context,
          error,
          committed_status,
        } => ParseResult::failed(parse_context, error, committed_status),
      };
      println!("end: collect");
      r
    })
  }
}

impl<'a, T, I: 'a, A> CollectParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}

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
