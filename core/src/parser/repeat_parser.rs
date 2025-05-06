use std::fmt::Debug;
use crate::prelude::*;
use crate::util::{Bound, RangeArgument};

pub trait RepeatParser<'a, I: 'a, A>: ParserRunner<'a, I, A>
where
  Self: 'a, {
  fn repeat<R>(
    self,
    range: R,
  ) -> Parser<'a, I, Vec<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Vec<A>> + 'a>
  where
    Self: 'a,
    I: Debug,
    R: RangeArgument<usize> + 'a,
    A: 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(range, none_separator)
  }

  fn of_many0(self) -> Parser<'a, I, Vec<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Vec<A>> + 'a>
  where
    Self: 'a,
    I: Debug,
    A: 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(0.., none_separator)
  }

  fn of_many1(self) -> Parser<'a, I, Vec<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Vec<A>> + 'a>
  where
    Self: 'a,
    I: Debug,
    A: 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(1.., none_separator)
  }

  fn count(
    self,
    count: usize,
  ) -> Parser<'a, I, Vec<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Vec<A>> + 'a>
  where
    Self: 'a,
    I: Debug,
    A: 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(count..=count, none_separator)
  }

  fn of_many0_sep<P2, B>(
    self,
    separator: P2,
  ) -> Parser<'a, I, Vec<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Vec<A>> + 'a>
  where
    Self: 'a,
    I: Debug,
    P2: ParserRunner<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    self.repeat_sep(0.., Some(separator))
  }

  fn of_many1_sep<P2, B>(
    self,
    separator: P2,
  ) -> Parser<'a, I, Vec<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Vec<A>> + 'a>
  where
    Self: 'a,
    I: Debug,
    P2: ParserRunner<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    self.repeat_sep(1.., Some(separator))
  }

  fn repeat_sep<P2, B, R>(
    self,
    range: R,
    separator_opt: Option<P2>,
  ) -> Parser<'a, I, Vec<A>, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, Vec<A>> + 'a>
  where
    I: Debug,
    R: RangeArgument<usize> + 'a,
    A: 'a,
    B: 'a,
    P2: ParserRunner<'a, I, B> + 'a, {
    let range_capture = range;

    Parser::new(move |parse_context| {
      let mut all_length = 0;
      let mut items = vec![];
      let range = &range_capture;

      // 最初のパース
      let first_result = self.run(parse_context.with_same_state());

      match first_result {
        ParseResult::Success {
          parse_context: pc1,
          value,
          length,
        } => {
          println!("length:{}", length);
          let mut current_parse_context = pc1.advance(length);
          items.push(value);
          all_length += length;

          // メインループ
          loop {
            let should_break = match range.end() {
              Bound::Included(&max_count) => items.len() >= max_count,
              Bound::Excluded(&max_count) => items.len() + 1 >= max_count,
              _ => false,
            };
            if should_break {
              break;
            }

            // セパレーターのパース
            let mut sep_success = true;
            let mut sep_length = 0;

            if let Some(ref sep) = separator_opt {
              // 新しいパースコンテキストを生成（with_same_stateを使用）
              let sep_parse_context = current_parse_context.with_same_state();
              let sep_result = sep.run(sep_parse_context);

              match sep_result {
                ParseResult::Success {
                  parse_context: pc2,
                  length,
                  ..
                } => {
                  current_parse_context = pc2.advance(length);
                  all_length += length;
                  sep_length = length;
                }
                _ => {
                  sep_success = false;
                }
              }
            }

            if !sep_success {
              break;
            }

            // 次の要素をパース - 新しいパースコンテキストを生成
            let next_parse_context = current_parse_context.with_same_state();
            let next_result = self.run(next_parse_context);

            match next_result {
              ParseResult::Success {
                parse_context: pc3,
                value,
                length,
              } => {
                current_parse_context = pc3.advance(length);
                items.push(value);
                all_length += length;
              }
              _ => {
                if sep_length > 0 {
                  all_length -= sep_length;
                }
                break;
              }
            }
          }

          if let Bound::Included(&min_count) = range.start() {
            if items.len() < min_count {
              let pc = parse_context.advance(all_length);
              let input = pc.input();
              let offset = pc.offset();
              let pe = ParseError::of_mismatch(
                input,
                offset,
                all_length,
                format!(
                  "expect repeat at least {} times, found {} times",
                  min_count,
                  items.len()
                ),
              );
              return ParseResult::failed(pc, pe, CommittedStatus::Uncommitted);
            }
          }

          ParseResult::successful(parse_context, items, all_length)
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

impl<'a, T, I: 'a, A> RepeatParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + ParserMonad<'a, I, A> {}

// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[test]
//   fn test_basic_repeat() {
//     // テストデータ
//     let text = "aaab";
//     let input: Vec<char> = text.chars().collect();
//
//     // 文字「a」を認識するクローン可能なパーサー
//     let a_parser = elm_ref('a');
//
//     // 0回以上の繰り返し
//     let many_a = a_parser.of_many0();
//
//     // パース実行
//     let result = many_a.parse(&input);
//
//     // 結果を検証
//     assert!(result.is_success());
//     if let ParseResult::Success { value, length, .. } = result {
//       assert_eq!(value.len(), 3); // 「a」が3回見つかるはず
//       assert_eq!(length, 3); // 消費される長さは3
//     } else {
//       panic!("Expected success but got failure");
//     }
//   }
//
//   #[test]
//   fn test_repeat_with_separator() {
//     // テスト用の入力文字列
//     let text = "a,a,a,b";
//     let input: Vec<char> = text.chars().collect();
//
//     // 文字認識用パーサー - クローン可能なものを使用
//     let a_parser = elm_ref('a');
//     let comma_parser = elm_ref(',');
//
//     // カンマ区切りのリスト
//     let a_comma_list = a_parser.of_many1_sep(comma_parser);
//
//     // パース実行
//     let result = a_comma_list.parse(&input);
//
//     // 結果を検証
//     assert!(result.is_success());
//     if let ParseResult::Success { value, length, .. } = result {
//       assert_eq!(value.len(), 3); // 「a」が3つあるはず
//       assert_eq!(length, 5); // 「a,a,a」で長さ5
//     } else {
//       panic!("Expected success but got failure");
//     }
//   }
//
//   #[test]
//   fn test_exact_repeat_count() {
//     // テスト用の入力文字列
//     let text = "aaaa";
//     let input: Vec<char> = text.chars().collect();
//
//     // 文字「a」を認識するクローン可能なパーサー
//     let a_parser = elm_ref('a');
//
//     // ちょうど3回の繰り返し
//     let exactly_three_a = a_parser.count(3);
//
//     // パース実行
//     let result = exactly_three_a.parse(&input);
//
//     // 結果を検証
//     assert!(result.is_success());
//     if let ParseResult::Success { value, length, .. } = result {
//       assert_eq!(value.len(), 3); // 要素数は3
//       assert_eq!(length, 3); // 消費された長さも3
//     } else {
//       panic!("Expected success but got failure");
//     }
//   }
// }
