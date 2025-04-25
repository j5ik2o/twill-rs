// 基本的なパーサーライブラリ
#[derive(Debug)]
pub enum PResult<I, O, E> {
  Ok(O, I),
  Err(E, bool), // bool = committed?
}

// 基本的なパーサートレイト
pub trait Parser<I, O, E> {
  fn parse(&self, input: &I) -> PResult<I, O, E>;
}

// クロージャをパーサーとして扱う
impl<F, I, O, E> Parser<I, O, E> for F
where
  F: Fn(&I) -> PResult<I, O, E>,
{
  fn parse(&self, input: &I) -> PResult<I, O, E> {
    self(input)
  }
}

// パーサー演算子を提供するトレイト
pub trait OperatorParser<I, O, E>: Parser<I, O, E> + Sized {
  // パーサーを選択的に適用（選言）
  fn or<P>(self, alt: P) -> impl Parser<I, O, E>
  where
    P: Parser<I, O, E>, {
    move |input: &I| match self.parse(input) {
      PResult::Err(e, true) => PResult::Err(e, true),
      PResult::Err(_, false) => alt.parse(input),
      ok @ PResult::Ok(..) => ok,
    }
  }

  // 連続パーサー（連言）- 標準版（Cloneが必要）
  fn and_then<P2, O2>(self, p2: P2) -> impl Parser<I, (O, O2), E>
  where
    I: Clone,
    O: Clone,
    P2: Parser<I, O2, E>, {
    move |input: &I| match self.parse(input) {
      PResult::Ok(o1, i1) => match p2.parse(&i1) {
        PResult::Ok(o2, i2) => PResult::Ok((o1.clone(), o2), i2),
        PResult::Err(e, c) => PResult::Err(e, c),
      },
      PResult::Err(e, c) => PResult::Err(e, c),
    }
  }
  
  // 連続パーサー（第1パーサーの結果を捨てる）- Clone回避
  fn skip_left<P2, O2>(self, p2: P2) -> impl Parser<I, O2, E>
  where
    I: Clone,
    P2: Parser<I, O2, E>, {
    move |input: &I| match self.parse(input) {
      PResult::Ok(_, i1) => p2.parse(&i1),
      PResult::Err(e, c) => PResult::Err(e, c),
    }
  }
  
  // 連続パーサー（第2パーサーの結果を捨てる）- Clone回避
  fn skip_right<P2>(self, p2: P2) -> impl Parser<I, O, E>
  where
    I: Clone,
    O: Clone,
    P2: Parser<I, (), E>, {
    move |input: &I| match self.parse(input) {
      PResult::Ok(o1, i1) => match p2.parse(&i1) {
        PResult::Ok(_, i2) => PResult::Ok(o1.clone(), i2),
        PResult::Err(e, c) => PResult::Err(e, c),
      },
      PResult::Err(e, c) => PResult::Err(e, c),
    }
  }
  
  // 結果を捨てて()を返す
  fn discard(self) -> impl Parser<I, (), E>
  where
    I: Clone, {
    move |input: &I| match self.parse(input) {
      PResult::Ok(_, i) => PResult::Ok((), i),
      PResult::Err(e, c) => PResult::Err(e, c),
    }
  }
}

// パーサーの変換メソッドを提供するトレイト
pub trait ParserExt<I, O, E>: Parser<I, O, E> + Sized {
  // 成功結果を変換
  fn map<F, O2>(self, f: F) -> impl Parser<I, O2, E>
  where
    F: Fn(O) -> O2, {
    move |input: &I| match self.parse(input) {
      PResult::Ok(o, i) => PResult::Ok(f(o), i),
      PResult::Err(e, c) => PResult::Err(e, c),
    }
  }

  // パーサー連鎖
  fn flat_map<F, P, O2>(self, f: F) -> impl Parser<I, O2, E>
  where
    F: Fn(O) -> P,
    P: Parser<I, O2, E>, {
    move |input: &I| match self.parse(input) {
      PResult::Ok(o, i) => f(o).parse(&i),
      PResult::Err(e, c) => PResult::Err(e, c),
    }
  }
}

// すべてのパーサーに拡張メソッドを提供
impl<T, I, O, E> ParserExt<I, O, E> for T where T: Parser<I, O, E> {}

// すべてのパーサーに演算子メソッドを提供
impl<T, I, O, E> OperatorParser<I, O, E> for T where T: Parser<I, O, E> {}

// 常に成功するパーサー
pub fn pure<I: Clone, O: Clone, E>(value: O) -> impl Parser<I, O, E> {
  let value = value.clone();
  move |input: &I| PResult::Ok(value.clone(), input.clone())
}

// 何もしないパーサー - 入力を消費せず値も返さない
pub fn empty<I: Clone, E>() -> impl Parser<I, (), E> {
  move |input: &I| PResult::Ok((), input.clone())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Clone)]
  struct TestError;

  #[test]
  fn test_pure() {
    let p = pure::<String, i32, TestError>(42);
    let input = "hello".to_string();

    match p.parse(&input) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, 42);
        assert_eq!(rest, "hello");
      }
      _ => panic!("Pure parser should always succeed"),
    }
  }

  #[test]
  fn test_map() {
    let p = pure::<String, i32, TestError>(10);
    let mapped = p.map(|x| x * 3);
    match mapped.parse(&"input".to_string()) {
      PResult::Ok(val, _) => assert_eq!(val, 30),
      _ => panic!("Map should transform the output value"),
    }
  }

  #[test]
  fn test_flat_map() {
    let p = pure::<String, i32, TestError>(5);
    let result = p.flat_map(|x| pure::<String, String, TestError>(format!("result: {}", x)));
    match result.parse(&"input".to_string()) {
      PResult::Ok(val, _) => assert_eq!(val, "result: 5"),
      _ => panic!("flat_map should chain parsers"),
    }
  }

  #[test]
  fn test_choice() {
    let p1 = pure::<String, &'static str, TestError>("first");
    let p2 = pure::<String, &'static str, TestError>("second");
    let combined = p1.or(p2);
    match combined.parse(&"input".to_string()) {
      PResult::Ok(val, _) => assert_eq!(val, "first"),
      _ => panic!("Or should return first result on success"),
    }
  }

  #[test]
  fn test_and_then() {
    // 二つの文字列を連続してパースするテスト
    let p1 = pure::<String, &'static str, TestError>("hello");
    let p2 = pure::<String, &'static str, TestError>("world");

    let combined = p1.and_then(p2);

    match combined.parse(&"input".to_string()) {
      PResult::Ok((first, second), rest) => {
        assert_eq!(first, "hello");
        assert_eq!(second, "world");
        assert_eq!(rest, "input");
      }
      _ => panic!("and_then should combine two parsers"),
    }
  }
  
  #[test]
  fn test_skip_left() {
    // 二つの文字列を連続してパースするテスト（最初の結果を捨てる）
    let p1 = pure::<String, &'static str, TestError>("hello");
    let p2 = pure::<String, &'static str, TestError>("world");

    let combined = p1.skip_left(p2);

    match combined.parse(&"input".to_string()) {
      PResult::Ok(result, rest) => {
        assert_eq!(result, "world"); // 最初の結果"hello"は捨てられる
        assert_eq!(rest, "input");
      }
      _ => panic!("skip_left should use only the second parser's result"),
    }
  }

  #[test]
  fn test_and_then_with_error() {
    // 最初のパーサーは成功するが、2番目のパーサーが失敗するケース
    let success_parser = pure::<String, &'static str, TestError>("ok");

    // 常に失敗するパーサー
    let failure_parser =
      move |_: &String| -> PResult<String, &'static str, TestError> { PResult::Err(TestError, false) };

    let combined = success_parser.and_then(failure_parser);

    match combined.parse(&"test".to_string()) {
      PResult::Err(_, committed) => {
        assert!(!committed, "Error from second parser should not be committed");
      }
      _ => panic!("and_then should fail when second parser fails"),
    }
  }
  
  #[test]
  fn test_skip_right() {
    // 最初のパーサーの結果だけを保持するテスト
    let p1 = pure::<String, &'static str, TestError>("hello");
    let p2 = empty::<String, TestError>();
    
    let combined = p1.skip_right(p2);
    
    match combined.parse(&"input".to_string()) {
      PResult::Ok(result, rest) => {
        assert_eq!(result, "hello");
        assert_eq!(rest, "input");
      }
      _ => panic!("skip_right should keep only the first parser's result"),
    }
  }
  
  #[test]
  fn test_discard() {
    // 結果を捨てて()を返すテスト
    let p = pure::<String, &'static str, TestError>("hello");
    let discarded = p.discard();
    
    match discarded.parse(&"input".to_string()) {
      PResult::Ok(result, rest) => {
        // 結果が()であることを確認
        assert_eq!(result, ());
        // 入力は維持される
        assert_eq!(rest, "input");
      }
      _ => panic!("discard should return unit value ()"),
    }
    
    // 複数のパーサーとの組み合わせテスト
    let p1 = pure::<String, &'static str, TestError>("hello");
    let p2 = pure::<String, &'static str, TestError>("world");
    
    // p1の結果を捨て、p2を実行
    let combined = p1.discard().skip_left(p2);
    
    match combined.parse(&"input".to_string()) {
      PResult::Ok(result, rest) => {
        assert_eq!(result, "world");
        assert_eq!(rest, "input");
      }
      _ => panic!("discard combined with skip_left should return p2's result"),
    }
  }
}
