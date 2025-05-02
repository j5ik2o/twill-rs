use crate::core::FnParser;
use crate::core::{ClonableParser, ParseContext, ParseError, ParseResult};

// Parser that matches a specific string
pub fn string<'a>(expected: &'static str) -> impl ClonableParser<'a, char, &'static str> {
  FnParser::new(move |mut parse_context: ParseContext<'a, char>| {
    let input = parse_context.input();
    let expected_chars: Vec<char> = expected.chars().collect();
    if input.len() >= expected_chars.len() && input[..expected_chars.len()] == expected_chars[..] {
      parse_context.advance_mut(expected_chars.len());
      ParseResult::successful(parse_context, expected, expected_chars.len())
    } else {
      let error_msg = format!("Expected '{}', but got something else", expected);
      let length = input.len().min(expected_chars.len());
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(parse_context, length, error_msg))
    }
  })
}

// Parser that matches any character
pub fn any_char<'a>() -> impl ClonableParser<'a, char, char> {
  FnParser::new(move |mut parse_context: ParseContext<'a, char>| {
    let input = parse_context.input();
    if let Some(&c) = input.get(0) {
      parse_context.next_mut();
      ParseResult::successful(parse_context, c, 1)
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(parse_context, 0, "Input is empty".to_string()))
    }
  })
}

// Parser that matches one of the given characters
pub fn one_of<'a>(chars: &'static [char]) -> impl ClonableParser<'a, char, char> {
  FnParser::new(move |mut parse_context: ParseContext<'a, char>| {
    let input = parse_context.input();
    if let Some(&c) = input.get(0) {
      if chars.contains(&c) {
        parse_context.next_mut();
        ParseResult::successful(parse_context, c, 1)
      } else {
        let error_msg = format!("Expected one of {:?}, but got '{}'", chars, c);
        ParseResult::failed_with_uncommitted(ParseError::of_mismatch(parse_context, 1, error_msg))
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_mismatch(parse_context, 0, "Input is empty".to_string()))
    }
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::CommittedStatus;
  use crate::core::Parser;

  // Helper function to create a ParseContext
  fn create_context(s: &'static str) -> ParseContext<'static, char> {
    let chars: Vec<char> = s.chars().collect();
    let slice = Box::leak(chars.into_boxed_slice());
    ParseContext::new(slice, 0)
  }

  #[test]
  fn test_string_parser() {
    // Success case
    let context = create_context("hello world");
    match string("hello").run(context) {
      ParseResult::Success {
        parse_context: new_context,
        value,
        ..
      } => {
        assert_eq!(value, "hello");
        assert_eq!(new_context.offset(), 5);
      }
      _ => panic!("String parser should match"),
    }

    // Failure case
    let context = create_context("goodbye");
    match string("hello").run(context) {
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
    let context = create_context("abc");
    match any_char().run(context) {
      ParseResult::Success {
        parse_context: new_context,
        value,
        ..
      } => {
        assert_eq!(value, 'a');
        assert_eq!(new_context.offset(), 1);
      }
      _ => panic!("Any char parser should succeed on non-empty input"),
    }
  }

  #[test]
  fn test_one_of() {
    let digits = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // Success case
    let context = create_context("5abc");
    match one_of(digits).run(context) {
      ParseResult::Success {
        parse_context: new_context,
        value,
        ..
      } => {
        assert_eq!(value, '5');
        assert_eq!(new_context.offset(), 1);
      }
      _ => panic!("one_of parser should match on valid input"),
    }

    // Failure case
    let context = create_context("abc");
    match one_of(digits).run(context) {
      ParseResult::Failure { .. } => {}
      _ => panic!("one_of parser should fail on invalid input"),
    }
  }
}
