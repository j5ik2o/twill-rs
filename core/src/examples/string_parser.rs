use crate::core::{ParseError, ParseResult, Parser, pure, OperatorParser, ParserExt};

// Parser that matches a specific string
pub fn string<'a>(expected: &'static str) -> impl Parser<'a, char, &'static str> {
  move |input: &'a [char]| {
    let expected_chars: Vec<char> = expected.chars().collect();
    if input.len() >= expected_chars.len() && input[..expected_chars.len()] == expected_chars[..] {
      ParseResult::successful(expected, expected_chars.len())
    } else {
      let error_msg = format!("Expected '{}', but got something else", expected);
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
        input,
        0,
        input.len().min(expected_chars.len()),
        error_msg,
      ))
    }
  }
}

// Parser that matches any character
pub fn any_char<'a>() -> impl Parser<'a, char, char> {
  move |input: &'a [char]| {
    if let Some(&c) = input.get(0) {
      ParseResult::successful(c, 1)
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(input, 0, 0, "Input is empty".to_string()))
    }
  }
}

// Parser that matches one of the given characters
pub fn one_of<'a>(chars: &'static [char]) -> impl Parser<'a, char, char> {
  move |input: &'a [char]| {
    if let Some(&c) = input.get(0) {
      if chars.contains(&c) {
        ParseResult::successful(c, 1)
      } else {
        let error_msg = format!("Expected one of {:?}, but got '{}'", chars, c);
        ParseResult::failed_with_uncommitted(ParseError::of_mismatch(input, 0, 1, error_msg))
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(input, 0, 0, "Input is empty".to_string()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::CommittedStatus;

  // Helper function to create a static input
  fn static_input(s: &'static str) -> &'static [char] {
    Box::leak(s.chars().collect::<Vec<_>>().into_boxed_slice())
  }

  #[test]
  fn test_string_parser() {
    let hello = string("hello");

    // Success case
    let input = static_input("hello world");
    match hello.parse(input) {
      ParseResult::Success { value, length } => {
        assert_eq!(value, "hello");
        assert_eq!(length, 5);
      }
      _ => panic!("String parser should match"),
    }

    // Failure case
    let input = static_input("goodbye");
    match hello.parse(input) {
      ParseResult::Failure { committed_status, .. } => {
        assert_eq!(
          committed_status,
          CommittedStatus::Uncommitted,
          "Error should not be committed"
        );
      }
      _ => panic!("String parser should fail on mismatch"),
    }
  }

  #[test]
  fn test_any_char() {
    let p = any_char();

    let input = static_input("abc");
    match p.parse(input) {
      ParseResult::Success { value, length } => {
        assert_eq!(value, 'a');
        assert_eq!(length, 1);
      }
      _ => panic!("Any char parser should succeed on non-empty input"),
    }
  }

  #[test]
  fn test_one_of() {
    let digits = one_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

    // Success case
    let input = static_input("5abc");
    match digits.parse(input) {
      ParseResult::Success { value, length } => {
        assert_eq!(value, '5');
        assert_eq!(length, 1);
      }
      _ => panic!("one_of parser should match on valid input"),
    }

    // Failure case
    let input = static_input("abc");
    match digits.parse(input) {
      ParseResult::Failure { .. } => {}
      _ => panic!("one_of parser should fail on invalid input"),
    }
  }
}
