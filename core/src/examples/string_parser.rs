// 基本的な文字列パーサーの例
use crate::core::{PResult, Parser};

// 特定の文字列にマッチするパーサー
pub fn string<E: Clone>(expected: &'static str) -> impl Parser<String, &'static str, E>
where
  E: From<String>, {
  move |input: &String| {
    if input.starts_with(expected) {
      let rest = input[expected.len()..].to_string();
      PResult::Ok(expected, rest)
    } else {
      let error_msg = format!("Expected '{}', but got '{}'", expected, input);
      PResult::Err(E::from(error_msg), false)
    }
  }
}

// 1文字にマッチするパーサー
pub fn any_char<E: Clone>() -> impl Parser<String, char, E>
where
  E: From<String>, {
  move |input: &String| {
    if let Some(c) = input.chars().next() {
      let rest = input[c.len_utf8()..].to_string();
      PResult::Ok(c, rest)
    } else {
      PResult::Err(E::from("Input is empty".to_string()), false)
    }
  }
}

// いずれかの文字にマッチするパーサー
pub fn one_of<E: Clone>(chars: &'static [char]) -> impl Parser<String, char, E>
where
  E: From<String>, {
  move |input: &String| {
    if let Some(c) = input.chars().next() {
      if chars.contains(&c) {
        let rest = input[c.len_utf8()..].to_string();
        PResult::Ok(c, rest)
      } else {
        let error_msg = format!("Expected one of {:?}, but got '{}'", chars, c);
        PResult::Err(E::from(error_msg), false)
      }
    } else {
      PResult::Err(E::from("Input is empty".to_string()), false)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::{pure, OperatorParser, ParserExt};

  // 簡単なエラー型
  #[derive(Debug, PartialEq, Clone)]
  struct ParseError(String);

  impl From<String> for ParseError {
    fn from(s: String) -> Self {
      ParseError(s)
    }
  }

  impl Default for ParseError {
    fn default() -> Self {
      ParseError("Default error".to_string())
    }
  }

  #[test]
  fn test_string_parser() {
    let hello = string::<ParseError>("hello");

    // 成功ケース
    match hello.parse(&"hello world".to_string()) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, "hello");
        assert_eq!(rest, " world");
      }
      _ => panic!("String parser should match"),
    }

    // 失敗ケース
    match hello.parse(&"goodbye".to_string()) {
      PResult::Err(_, committed) => {
        assert!(!committed, "Error should not be committed");
      }
      _ => panic!("String parser should fail on mismatch"),
    }
  }

  #[test]
  fn test_any_char() {
    let p = any_char::<ParseError>();

    match p.parse(&"abc".to_string()) {
      PResult::Ok(c, rest) => {
        assert_eq!(c, 'a');
        assert_eq!(rest, "bc");
      }
      _ => panic!("Any char parser should succeed on non-empty input"),
    }
  }

  #[test]
  fn test_one_of() {
    let digits = one_of::<ParseError>(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

    // 成功ケース
    match digits.parse(&"5abc".to_string()) {
      PResult::Ok(c, rest) => {
        assert_eq!(c, '5');
        assert_eq!(rest, "abc");
      }
      _ => panic!("one_of parser should match on valid input"),
    }

    // 失敗ケース
    match digits.parse(&"abc".to_string()) {
      PResult::Err(_, _) => {}
      _ => panic!("one_of parser should fail on invalid input"),
    }
  }

  #[test]
  fn test_map() {
    let p = any_char::<ParseError>();
    let mapped = p.map(|c| c.to_uppercase().next().unwrap());

    match mapped.parse(&"abc".to_string()) {
      PResult::Ok(c, rest) => {
        assert_eq!(c, 'A');
        assert_eq!(rest, "bc");
      }
      _ => panic!("Map should transform the output value"),
    }
  }

  #[test]
  fn test_or_combinator() {
    let a = string::<ParseError>("hello");
    let b = string::<ParseError>("world");
    let combined = a.or(b);

    // 最初のパーサーが成功
    match combined.parse(&"hello test".to_string()) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, "hello");
        assert_eq!(rest, " test");
      }
      _ => panic!("Or combinator should succeed with first parser"),
    }

    // 最初のパーサーが失敗、2番目が成功
    let a = string::<ParseError>("hello");
    let b = string::<ParseError>("world");
    let combined = a.or(b);

    match combined.parse(&"world test".to_string()) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, "world");
        assert_eq!(rest, " test");
      }
      _ => panic!("Or combinator should succeed with second parser"),
    }
  }

  #[test]
  fn test_flat_map() {
    // 文字をパースして、その文字に基づいて別のパーサーを返す
    let p = any_char::<ParseError>().flat_map(|c| {
      if c.is_alphabetic() {
        // アルファベットの場合は「文字です」を返す
        pure("文字です".to_string())
      } else if c.is_numeric() {
        // 数字の場合は「数字です」を返す
        pure("数字です".to_string())
      } else {
        // それ以外の場合は「その他です」を返す
        pure("その他です".to_string())
      }
    });

    match p.parse(&"a123".to_string()) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, "文字です");
        assert_eq!(rest, "123");
      }
      _ => panic!("flat_map should chain parsers correctly"),
    }

    match p.parse(&"1abc".to_string()) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, "数字です");
        assert_eq!(rest, "abc");
      }
      _ => panic!("flat_map should chain parsers correctly"),
    }

    match p.parse(&"!abc".to_string()) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, "その他です");
        assert_eq!(rest, "abc");
      }
      _ => panic!("flat_map should chain parsers correctly"),
    }
  }

  #[test]
  fn test_pure() {
    let p = pure::<String, i32, ParseError>(42);

    match p.parse(&"unchanged".to_string()) {
      PResult::Ok(val, rest) => {
        assert_eq!(val, 42);
        assert_eq!(rest, "unchanged");
      }
      _ => panic!("Pure parser should always succeed"),
    }
  }
  
  #[test]
  fn test_and_then() {
    // 連続した文字列のパース
    let hello = string::<ParseError>("hello");
    let world = string::<ParseError>("world");
    
    // "hello world" を順番にパースする
    let hello_world = hello.and_then(world);
    
    // 成功ケース
    match hello_world.parse(&"helloworld".to_string()) {
      PResult::Ok((first, second), rest) => {
        assert_eq!(first, "hello");
        assert_eq!(second, "world");
        assert_eq!(rest, "");
      }
      _ => panic!("and_then should parse both strings successfully"),
    }
    
    // 失敗ケース（最初のパーサーは成功するが、2番目が失敗）
    match hello_world.parse(&"hello test".to_string()) {
      PResult::Err(_, _) => {}
      _ => panic!("and_then should fail when second parser fails"),
    }
  }
  
  #[test]
  fn test_complex_parsing() {
    // 数字の後にアルファベット文字が続くパターンをパースする例
    let digit = one_of::<ParseError>(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let alpha = move |input: &String| -> PResult<String, char, ParseError> {
      if let Some(c) = input.chars().next() {
        if c.is_alphabetic() {
          let rest = input[c.len_utf8()..].to_string();
          PResult::Ok(c, rest)
        } else {
          PResult::Err(ParseError(format!("Expected alphabetic character, but got '{}'", c)), false)
        }
      } else {
        PResult::Err(ParseError("Input is empty".to_string()), false)
      }
    };
    
    // 数字とアルファベットのペアをパース
    let digit_alpha = digit.and_then(alpha);
    
    // 成功ケース
    match digit_alpha.parse(&"5a123".to_string()) {
      PResult::Ok((d, a), rest) => {
        assert_eq!(d, '5');
        assert_eq!(a, 'a');
        assert_eq!(rest, "123");
      }
      _ => panic!("Complex parsing should match digit followed by alpha"),
    }
    
    // 失敗ケース（数字の後に数字）
    match digit_alpha.parse(&"55abc".to_string()) {
      PResult::Err(_, _) => {}
      _ => panic!("Complex parsing should fail when digit is not followed by alpha"),
    }
    
    // 失敗ケース（アルファベットで始まる）
    match digit_alpha.parse(&"abc123".to_string()) {
      PResult::Err(_, _) => {}
      _ => panic!("Complex parsing should fail when input doesn't start with digit"),
    }
  }
}
