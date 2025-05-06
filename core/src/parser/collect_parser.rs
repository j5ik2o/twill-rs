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
      match self.run(parse_context) {
        ParseResult::Success {
          parse_context, length, ..
        } => {
          log::debug!("length: {}", length);
          log::debug!("parse_context: {:?}", parse_context);
          // 元の入力に対して、適切な位置から消費した長さ分のスライスを取得
          // オフセットが長さより小さい場合は、最初のオフセットは0
          let start_offset = if parse_context.next_offset() >= length {
            parse_context.next_offset() - length
          } else {
            0
          };
          let slice = parse_context.slice_with_offset_len(start_offset, length);
          ParseResult::successful(parse_context, slice, length)
        }
        ParseResult::Failure {
          parse_context,
          error,
          committed_status,
        } => ParseResult::failed(parse_context, error, committed_status),
      }
    })
  }
}

impl<'a, T, I: 'a, A> CollectParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}

#[cfg(test)]
mod tests {
  use std::env;
  use crate::prelude::*;

  #[test]
  fn test_collect_1() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let text: &str = "a";
    let input = text.chars().collect::<Vec<_>>();
    let p = elm_ref('a').collect();

    let result = p.parse(&input).to_result();
    log::debug!("{:?}", result);

    assert!(result.is_ok());
  }

  #[test]
  fn test_collect_2() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let text: &str = "ab";
    let input = text.chars().collect::<Vec<_>>();
    let p = (elm_ref('a')+elm_ref('b')).collect();

    let result = p.parse(&input).to_result();
    log::debug!("{:?}", result);

    assert!(result.is_ok());
  }
}
