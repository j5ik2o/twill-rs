use crate::core::parse_result::ParseResult;
use crate::core::parser::reusable_with_clone;
use crate::core::parser::OrParser;
use crate::core::parser::ParserMonad;
use crate::core::parser::RcParser;
use crate::core::parser::{FuncParser, Parser};
use crate::prelude::successful;

/// Trait providing binary operator related parser operations
pub trait BinaryOperatorParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + OrParser<'a, I, A>
where
  Self: 'a, {
  /// Right associative binary operator parsing
  fn chain_right1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: Parser<'a, I, OP> + Clone + 'a, {
    self.clone().flat_map(move |x| self.clone().rest_right1(op.clone(), x))
  }

  /// Left associative binary operator parsing
  fn chain_left1<P2, OP>(self, op: P2) -> impl Parser<'a, I, A> + Clone
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    self.clone().flat_map(move |x| self.clone().rest_left1(op.clone(), x))
  }

  /// Right associative binary operator parsing helper
  fn rest_right1<P2, OP>(self, op: P2, x: A) -> impl Parser<'a, I, A> + Clone
  where
    A: Clone + 'a,
    OP: Fn(A, A) -> A + Clone + 'a,
    P2: Parser<'a, I, OP> + 'a, {
    let default_value = x.clone();
    op.clone()
      .flat_map(move |f| {
        let default_value = x.clone();
        self.clone().map(move |y| f(default_value.clone(), y))
      })
      .or(successful(default_value.clone()))
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
