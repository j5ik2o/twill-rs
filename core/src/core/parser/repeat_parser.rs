use crate::core::committed_status::CommittedStatus;
use crate::core::parser::collect_parser::CollectParser;
use crate::core::parser::rc_parser::reusable_with_clone;
use crate::core::parser::FuncParser;
use crate::core::util::{Bound, RangeArgument};
use crate::core::{BinaryOperatorParser, ParseError, ParseResult, Parser, ParserMonad};

// 基本的なRepeatParserトレイト - Clone制約を追加
pub trait RepeatParser<'a, I: 'a, A>: Parser<'a, I, A> + BinaryOperatorParser<'a, I, A> + Sized + Clone
where
  Self: 'a, {
}

// すべてのパーサーに対して RepeatParser を実装
impl<'a, T, I: 'a, A> RepeatParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + Clone + 'a {}

// クローン可能なパーサー向けの拡張機能
pub trait CloneableRepeater<'a, I: 'a, A>: RepeatParser<'a, I, A> + Clone
where
  Self: 'a, {
  // 基本的な繰り返しパーサー - セパレーターなし
  fn repeat<R>(self, range: R) -> impl Parser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Clone + 'a,
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(range, none_separator)
  }

  // セパレーターなしでの0回以上の繰り返し
  fn of_many0(self) -> impl Parser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(0.., none_separator)
  }

  // セパレーターなしでの1回以上の繰り返し
  fn of_many1(self) -> impl Parser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(1.., none_separator)
  }

  // 指定回数の繰り返し
  fn count(self, count: usize) -> impl Parser<'a, I, Vec<A>>
  where
    A: Clone + 'a, {
    let none_separator: Option<Self> = None;
    self.repeat_sep(count..=count, none_separator)
  }

  // セパレーターありでの0回以上の繰り返し
  fn of_many0_sep<P2, B>(self, separator: P2) -> impl Parser<'a, I, Vec<A>>
  where
    P2: Parser<'a, I, B> + Clone + 'a,
    A: Clone + 'a,
    B: Clone + 'a, {
    self.repeat_sep(0.., Some(separator))
  }

  // セパレーターありでの1回以上の繰り返し
  fn of_many1_sep<P2, B>(self, separator: P2) -> impl Parser<'a, I, Vec<A>>
  where
    P2: Parser<'a, I, B> + Clone + 'a,
    A: Clone + 'a,
    B: Clone + 'a, {
    self.repeat_sep(1.., Some(separator))
  }

  // 任意範囲の繰り返し（セパレーターオプション付き）
  fn repeat_sep<P2, B, R>(self, range: R, separator_opt: Option<P2>) -> impl Parser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Clone + 'a,
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + 'a, {
    // パーサーはClone可能なので、rc_parserモジュールのreusable_with_cloneを使用
    // to_rc_parserは使い捨てパーサーで利用しにくいため廃止
    let main_parser = reusable_with_clone(self);
    let separator = separator_opt.map(reusable_with_clone);
    let range_capture = range;

    FuncParser::new(move |parse_context| {
      let mut all_length = 0;
      let mut items = vec![];
      let range = &range_capture;

      // 最初のパース
      let first_result = main_parser.clone().run(parse_context.with_same_state());

      match first_result {
        ParseResult::Success {
          parse_context: pc1,
          value,
          length,
        } => {
          println!("length:{}", length);
          let mut current_parse_state = pc1.advance(length);
          items.push(value);
          all_length += length;

          // メインループ
          loop {
            let should_break = match range.end() {
              Bound::Included(&max_count) => items.len() >= max_count,
              Bound::Excluded(&max_count) => items.len() + 1 >= max_count,
              _ => false,
            };
            println!("bBreak:{}", should_break);
            if should_break {
              break;
            }

            // セパレーターのパース
            let mut sep_success = true;
            let mut sep_length = 0;

            if let Some(ref sep) = separator {
              // 新しいパースコンテキストを生成（with_same_stateを使用）
              let sep_parse_state = current_parse_state.with_same_state();
              let sep_result = sep.clone().run(sep_parse_state);

              match sep_result {
                ParseResult::Success {
                  parse_context: pc2,
                  length,
                  ..
                } => {
                  println!("sep: length:{}", length);
                  current_parse_state = pc2.advance(length);
                  all_length += length;
                  sep_length = length;
                }
                _ => {
                  println!("sep: failed");
                  sep_success = false;
                }
              }
            }

            if !sep_success {
              break;
            }

            // 次の要素をパース - 新しいパースコンテキストを生成
            let next_parse_state = current_parse_state.with_same_state();
            let next_result = main_parser.clone().run(next_parse_state);

            match next_result {
              ParseResult::Success {
                parse_context: pc3,
                value,
                length,
              } => {
                println!("n: length:{}", length);
                current_parse_state = pc3.advance(length);
                items.push(value);
                all_length += length;
              }
              _ => {
                // 次の要素パースに失敗した場合は、セパレーターの結果をロールバック
                if sep_length > 0 {
                  all_length -= sep_length;
                }
                println!("n: failed");
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
impl<'a, T, I: 'a, A> CloneableRepeater<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + Clone + 'a {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::parse_context::ParseContext;
  use crate::core::parse_result::ParseResult;
  use crate::core::parser::rc_parser::RcParser;

  // 単純なクローン可能なパーサーを作成
  fn char_parser<'a>(c: char) -> impl Parser<'a, char, char> + Clone {
    // RcParserを直接使う（これはクローン可能）
    RcParser::new(move |parse_context: ParseContext<'a, char>| {
      let input = parse_context.input();
      if let Some(&actual) = input.get(0) {
        if actual == c {
          return ParseResult::successful(parse_context.with_same_state(), actual, 1);
        }
      }
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(parse_context, 0, format!("Expected {}", c)))
    })
  }

  #[test]
  fn test_basic_repeat() {
    // テストデータ
    let text = "aaab";
    let input: Vec<char> = text.chars().collect();

    // 文字「a」を認識するクローン可能なパーサー
    let a_parser = char_parser('a');

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
    let a_parser = char_parser('a');
    let comma_parser = char_parser(',');

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
    let a_parser = char_parser('a');

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
