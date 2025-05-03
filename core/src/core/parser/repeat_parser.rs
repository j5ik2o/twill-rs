use crate::core::committed_status::CommittedStatus;
use crate::core::parser::rc_parser::reusable_parser;
use crate::core::parser::FnParser;
use crate::core::parser::{BinaryOperatorParser, ClonableParser, ParseResult, ParserMonad};
use crate::core::util::{Bound, RangeArgument};
use crate::core::{ParseError, Parser};

// 基本的なRepeatParserトレイト - Clone制約を追加
pub trait RepeatParser<'a, I: 'a, A>: ClonableParser<'a, I, A> + BinaryOperatorParser<'a, I, A>
where
  Self: 'a, {
  // 基本的な繰り返しパーサー - セパレーターなし
  fn repeat<R>(self, range: R) -> impl ClonableParser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Clone + 'a,
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(range, none_separator)
  }

  // セパレーターなしでの0回以上の繰り返し
  fn of_many0(self) -> impl ClonableParser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(0.., none_separator)
  }

  // セパレーターなしでの1回以上の繰り返し
  fn of_many1(self) -> impl ClonableParser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(1.., none_separator)
  }

  // 指定回数の繰り返し
  fn count(self, count: usize) -> impl ClonableParser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(count..=count, none_separator)
  }

  // セパレーターありでの0回以上の繰り返し
  fn of_many0_sep<P2, B>(self, separator: P2) -> impl ClonableParser<'a, I, Vec<A>>
  where
    P2: ClonableParser<'a, I, B> + 'a,
    A: Clone + 'a,
    B: Clone + 'a, {
    self.repeat_sep(0.., Some(separator))
  }

  // セパレーターありでの1回以上の繰り返し
  fn of_many1_sep<P2, B>(self, separator: P2) -> impl ClonableParser<'a, I, Vec<A>>
  where
    P2: ClonableParser<'a, I, B> + 'a,
    A: Clone + 'a,
    B: Clone + 'a, {
    self.repeat_sep(1.., Some(separator))
  }

  // 任意範囲の繰り返し（セパレーターオプション付き）
  fn repeat_sep<P2, B, R>(self, range: R, separator_opt: Option<P2>) -> impl ClonableParser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Clone + 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: ClonableParser<'a, I, B> + 'a, {
    let parser_clone = self.clone();
    let main_parser_factory = move || parser_clone.clone();
    let main_parser = reusable_parser(main_parser_factory);

    let separator = separator_opt.map(|sep| {
      let sep_clone = sep.clone();
      reusable_parser(move || sep_clone.clone())
    });

    let range_capture = range;

    FnParser::new(move |parse_context| {
      let mut all_length = 0;
      let mut items = vec![];
      let range = &range_capture;

      // 最初のパース
      let first_result = main_parser.run(parse_context.with_same_state());

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

            if let Some(ref sep) = separator {
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
            let next_result = main_parser.run(next_parse_context);

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
                // 次の要素パースに失敗した場合は、セパレーターの結果をロールバック
                if sep_length > 0 {
                  all_length -= sep_length;
                }
                break;
              }
            }
          }

          // 最小繰り返し回数のチェック
          if let Bound::Included(&min_count) = range.start() {
            if items.len() < min_count {
              let pc = parse_context.advance(all_length);
              let pe = ParseError::of_mismatch(
                pc,
                all_length,
                format!(
                  "expect repeat at least {} times, found {} times",
                  min_count,
                  items.len()
                ),
              );
              return ParseResult::failed(pe, CommittedStatus::Uncommitted);
            }
          }

          ParseResult::successful(parse_context, items, all_length)
        }
        // 最初のパースが失敗した場合
        ParseResult::Failure {
          error,
          committed_status,
        } => ParseResult::failed(error, committed_status),
      }
    })
  }
}

// Cloneを実装したパーサーに対してCloneableRepeaterを実装
impl<'a, T, I: 'a, A> RepeatParser<'a, I, A> for T where T: ClonableParser<'a, I, A> + ParserMonad<'a, I, A> {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::parse_result::ParseResult;
  use crate::core::parser::combinators::elm_ref;

  #[test]
  fn test_basic_repeat() {
    // テストデータ
    let text = "aaab";
    let input: Vec<char> = text.chars().collect();

    // 文字「a」を認識するクローン可能なパーサー
    let a_parser = elm_ref('a');

    // 0回以上の繰り返し
    let many_a = a_parser.of_many0();

    // パース実行
    let result = many_a.parse(&input);

    // 結果を検証
    assert!(result.is_success());
    if let ParseResult::Success { value, length, .. } = result {
      assert_eq!(value.len(), 3); // 「a」が3回見つかるはず
      assert_eq!(length, 3); // 消費される長さは3
    } else {
      panic!("Expected success but got failure");
    }
  }

  #[test]
  fn test_repeat_with_separator() {
    // テスト用の入力文字列
    let text = "a,a,a,b";
    let input: Vec<char> = text.chars().collect();

    // 文字認識用パーサー - クローン可能なものを使用
    let a_parser = elm_ref('a');
    let comma_parser = elm_ref(',');

    // カンマ区切りのリスト
    let a_comma_list = a_parser.of_many1_sep(comma_parser);

    // パース実行
    let result = a_comma_list.parse(&input);

    // 結果を検証
    assert!(result.is_success());
    if let ParseResult::Success { value, length, .. } = result {
      assert_eq!(value.len(), 3); // 「a」が3つあるはず
      assert_eq!(length, 5); // 「a,a,a」で長さ5
    } else {
      panic!("Expected success but got failure");
    }
  }

  #[test]
  fn test_exact_repeat_count() {
    // テスト用の入力文字列
    let text = "aaaa";
    let input: Vec<char> = text.chars().collect();

    // 文字「a」を認識するクローン可能なパーサー
    let a_parser = elm_ref('a');

    // ちょうど3回の繰り返し
    let exactly_three_a = a_parser.count(3);

    // パース実行
    let result = exactly_three_a.parse(&input);

    // 結果を検証
    assert!(result.is_success());
    if let ParseResult::Success { value, length, .. } = result {
      assert_eq!(value.len(), 3); // 要素数は3
      assert_eq!(length, 3); // 消費された長さも3
    } else {
      panic!("Expected success but got failure");
    }
  }
}
