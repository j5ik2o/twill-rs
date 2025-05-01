use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::or_parser::OrParser;
use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::rc_parser::reusable_with_clone;
use crate::core::parser::{FuncParser, Parser};
use crate::core::{successful, RcParser};

/// Trait providing binary operator related parser operations
pub trait BinaryOperatorParser<'a, I: 'a, A>:
  Parser<'a, I, A> + ParserMonad<'a, I, A> + OrParser<'a, I, A>
where
  Self: 'a, {
  /// Right associative binary operator parsing
  fn chain_right1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: Parser<'a, I, OP> + Clone + 'a, {
    let parser_clone = self.clone();
    let op_clone = op.clone();

    reusable_with_clone(FuncParser::new(move |pc| {
      let first_result = reusable_with_clone(parser_clone.clone()).run(pc);
      match first_result {
        ParseResult::Success {
          parse_context,
          value,
          length,
        } => {
          let next_parser = self.clone().rest_right1(op_clone.clone(), value.clone());
          let next_result = next_parser.run(parse_context);
          match next_result {
            ParseResult::Success {
              parse_context,
              value,
              length: next_length,
            } => ParseResult::successful(parse_context, value, length + next_length),
            ParseResult::Failure { error, .. } => {
              ParseResult::successful(error.parse_context().with_same_state(), value, length)
            }
          }
        }
        failed => failed,
      }
    }))
  }

  /// Left associative binary operator parsing
  fn chain_left1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A> + Clone
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    let parser_clone = self.clone();
    let op_clone = op.clone();

    reusable_with_clone(FuncParser::new(move |pc| {
      let first_result = reusable_with_clone(parser_clone.clone()).run(pc);
      match first_result {
        ParseResult::Success {
          parse_context,
          value,
          length,
        } => {
          let next_parser = self.clone().rest_left1(op_clone.clone(), value.clone());
          let next_result = next_parser.run(parse_context);
          match next_result {
            ParseResult::Success {
              parse_context,
              value,
              length: next_length,
            } => ParseResult::successful(parse_context, value, length + next_length),
            ParseResult::Failure { error, .. } => {
              ParseResult::successful(error.parse_context().with_same_state(), value, length)
            }
          }
        }
        failed => failed,
      }
    }))
  }

  /// Right associative binary operator parsing helper
  fn rest_right1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A> + Clone
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    let default_value = x.clone();
    let self_clone = self.clone();
    let op_clone = op.clone();

    reusable_with_clone(FuncParser::new(move |pc| {
      let op_result = reusable_with_clone(op_clone.clone()).run(pc);

      match op_result {
        ParseResult::Success {
          parse_context,
          value: f,
          length: op_length,
        } => {
          let next_parser = reusable_with_clone(self_clone.clone());
          let expr_result = next_parser.run(parse_context);

          match expr_result {
            ParseResult::Success {
              parse_context,
              value: y,
              length: expr_length,
            } => {
              // 次の式の結果とデフォルト値を関数に適用
              let result = f(y, default_value.clone());
              ParseResult::successful(parse_context, result, op_length + expr_length)
            }
            ParseResult::Failure {
              error,
              committed_status,
            } => ParseResult::failed(error, committed_status),
          }
        }
        ParseResult::Failure { error, .. } => {
          // 演算子が見つからない場合はデフォルト値を返す
          ParseResult::successful(error.parse_context().with_same_state(), x.clone(), 0)
        }
      }
    }))
  }

  /// Left associative binary operator parsing helper with the default value
  ///
  /// This method takes an operator parser and a default value and
  /// returns a parser that repeatedly applies the left associative operation on
  /// the parsed values or returns the default value if no operations can be applied.
  fn rest_left1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A> + Clone
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    let self_clone = reusable_with_clone(self.clone());
    let op_clone = reusable_with_clone(op.clone());

    RcParser::new(move |pc| {
      // 左結合で連続して式を解析する
      let mut current_value = x.clone();
      let mut current_pc = pc.with_same_state();
      let mut total_length = 0;

      // 繰り返し処理
      loop {
        let op_result = op_clone.clone().run(current_pc.with_same_state());

        match op_result {
          ParseResult::Success {
            parse_context,
            value: f,
            length: op_length,
          } => {
            // 演算子の後には式が続くはず
            let expr_result = self_clone.clone().run(parse_context);

            match expr_result {
              ParseResult::Success {
                parse_context,
                value: y,
                length: expr_length,
              } => {
                // 結果を更新して次のループへ
                current_value = f(current_value, y);
                current_pc = parse_context;
                total_length += op_length + expr_length;
              }
              ParseResult::Failure {
                error,
                committed_status,
              } => {
                // 式が見つからない場合は終了
                return ParseResult::failed(error, committed_status);
              }
            }
          }
          ParseResult::Failure { .. } => {
            // 演算子が見つからない場合はループを終了
            break;
          }
        }
      }

      // 最終結果を返す
      ParseResult::successful(pc, current_value, total_length)
    })
  }
}

/// Implement BinaryOperatorParser for all types that implement Parser, ParserMonad, ChoiceParser and Clone
impl<'a, T, I: 'a, A> BinaryOperatorParser<'a, I, A> for T where
  T: Parser<'a, I, A> + ParserMonad<'a, I, A> + OrParser<'a, I, A> + Clone + 'a
{
}
