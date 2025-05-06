use std::ops::Add;
use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{ParserRunner, Parser};

/// Trait providing sequence-related parser operations (consuming self)
pub trait AndThenParser<'a, I: 'a, A>: ParserMonad<'a, I, A> {
  /// Sequential parser (conjunction) - implemented directly using RcParser (consuming self)
  fn and_then<P2, B>(
    self,
    p2: P2,
  ) -> Parser<'a, I, (A, B), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, (A, B)> + 'a>
  where
    A: 'a,
    B: 'a,
    P2: ParserRunner<'a, I, B> + 'a {
    Parser::new(move |context| {
      let original_input = context.original_input();
      let initial_offset = context.next_offset();
      
      match self.run(context) {
        ParseResult::Success {
          value: a,
          length: length1,
          ..
        } => {
          let new_offset = initial_offset + length1;
          let new_context = ParseContext::new(original_input, new_offset);
          
          match p2.run(new_context) {
            ParseResult::Success {
              parse_context: context3,
              value: b,
              length: length2,
            } => {
              ParseResult::successful(context3, (a, b), length1 + length2)
            },
            ParseResult::Failure {
              parse_context: context3,
              error,
              committed_status,
            } => {
              ParseResult::failed(context3, error, committed_status)
            },
          }
        },
        ParseResult::Failure {
          parse_context,
          error,
          committed_status,
        } => {
          // 1番目のパーサーが失敗した場合
          ParseResult::failed(parse_context, error, committed_status)
        },
      }
    })
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> AndThenParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + ParserMonad<'a, I, A> + 'a {}

impl<'a, I, A, F, B, G> Add<Parser<'a, I, B, G>> for Parser<'a, I, A, F>
  where
    A: Clone + 'a,
    B: Clone + 'a,
    F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
    G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a,
{
  type Output = Parser<'a, I, (A, B), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, (A, B)> + 'a>;

  fn add(self, rhs: Parser<'a, I, B, G>) -> Self::Output {
    self.and_then(rhs)
  }
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  pub fn test_and_then() {
    let text: &str = "xy";
    let input: Vec<char> = text.chars().collect::<Vec<_>>();

    // 個別のパーサーをテスト
    let parser_x = elm('x');
    let result_x = parser_x.parse(&input);
    println!("Parser X result: {:?}", result_x);
    assert!(result_x.is_success());
    
    let parser_y = elm('y');
    let input_y = &input[1..]; // 'y'だけの配列
    let result_y = parser_y.parse(input_y);
    println!("Parser Y result: {:?}", result_y);
    assert!(result_y.is_success());

    // and_thenを使った複合パーサー
    let parser = elm('x').and_then(elm('y'));
    let result = parser.parse(&input);
    println!("Combined parser result: {:?}", result);

    assert!(result.is_success());
    assert_eq!(result.success().unwrap(), ('x', 'y'));
  }
}
