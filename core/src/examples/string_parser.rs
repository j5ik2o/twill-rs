use crate::core::{ParseContext, ParseError, ParseResult, Parser};

// Parser that matches a specific string
pub fn string<'a>(expected: &'static str) -> impl Parser<'a, char, &'static str> {
  move |context: &ParseContext<'a, char>| {
    let input = context.input();
    let expected_chars: Vec<char> = expected.chars().collect();
    if input.len() >= expected_chars.len() && input[..expected_chars.len()] == expected_chars[..] {
      let new_context = context.advance(expected_chars.len());
      ParseResult::successful(expected, new_context)
    } else {
      let error_msg = format!("Expected '{}', but got something else", expected);
      let length = input.len().min(expected_chars.len());
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(context.clone(), length, error_msg))
    }
  }
}

// Parser that matches any character
pub fn any_char<'a>() -> impl Parser<'a, char, char> {
  move |context: &ParseContext<'a, char>| {
    let input = context.input();
    if let Some(&c) = input.get(0) {
      let new_context = context.next();
      ParseResult::successful(c, new_context)
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
        context.clone(),
        0,
        "Input is empty".to_string(),
      ))
    }
  }
}

// Parser that matches one of the given characters
pub fn one_of<'a>(chars: &'static [char]) -> impl Parser<'a, char, char> {
  move |context: &ParseContext<'a, char>| {
    let input = context.input();
    if let Some(&c) = input.get(0) {
      if chars.contains(&c) {
        let new_context = context.next();
        ParseResult::successful(c, new_context)
      } else {
        let error_msg = format!("Expected one of {:?}, but got '{}'", chars, c);
        ParseResult::failed_with_uncommitted(ParseError::of_mismatch(context.clone(), 1, error_msg))
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(
        context.clone(),
        0,
        "Input is empty".to_string(),
      ))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::CommittedStatus;

  // Helper function to create a ParseContext
  fn create_context(s: &'static str) -> ParseContext<'static, char> {
    let chars: Vec<char> = s.chars().collect();
    let slice = Box::leak(chars.into_boxed_slice());
    ParseContext::new(slice, 0)
  }

  #[test]
  fn test_string_parser() {
    let hello = string("hello");

    // Success case
    let context = create_context("hello world");
    match hello.parse(&context) {
      ParseResult::Success {
        value,
        context: new_context,
      } => {
        assert_eq!(value, "hello");
        assert_eq!(new_context.next_offset(), 5);
      }
      _ => panic!("String parser should match"),
    }

    // Failure case
    let context = create_context("goodbye");
    match hello.parse(&context) {
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

    let context = create_context("abc");
    match p.parse(&context) {
      ParseResult::Success {
        value,
        context: new_context,
      } => {
        assert_eq!(value, 'a');
        assert_eq!(new_context.next_offset(), 1);
      }
      _ => panic!("Any char parser should succeed on non-empty input"),
    }
  }

  #[test]
  fn test_one_of() {
    let digits = one_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

    // Success case
    let context = create_context("5abc");
    match digits.parse(&context) {
      ParseResult::Success {
        value,
        context: new_context,
      } => {
        assert_eq!(value, '5');
        assert_eq!(new_context.next_offset(), 1);
      }
      _ => panic!("one_of parser should match on valid input"),
    }

    // Failure case
    let context = create_context("abc");
    match digits.parse(&context) {
      ParseResult::Failure { .. } => {}
      _ => panic!("one_of parser should fail on invalid input"),
    }
  }
}
