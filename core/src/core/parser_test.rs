use crate::core::{ParseError, ParseResult, CommittedStatus};
use crate::core::parser::{Parser, pure, empty};

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Clone)]
  struct TestError;

  impl<'a> From<TestError> for ParseError<'a, char> {
    fn from(error: TestError) -> Self {
      ParseError::of_custom(0, None, "Test error".to_string())
    }
  }

  #[test]
  fn test_pure() {
    let p = pure::<char, i32>(42);
    let input = "hello".chars().collect::<Vec<_>>();

    match p.parse(&input) {
      ParseResult::Success { value, length } => {
        assert_eq!(value, 42);
        assert_eq!(length, 0); // pure doesn't consume input
      }
      _ => panic!("Pure parser should always succeed"),
    }
  }

  #[test]
  fn test_map() {
    let p = pure::<char, i32>(10);
    let mapped = p.map(|x| x * 3);
    match mapped.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value, .. } => assert_eq!(value, 30),
      _ => panic!("Map should transform the output value"),
    }
  }

  #[test]
  fn test_flat_map() {
    let p = pure::<char, i32>(5);
    let result = p.flat_map(|x| pure::<char, String>(format!("result: {}", x)));
    match result.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value, .. } => assert_eq!(value, "result: 5"),
      _ => panic!("flat_map should chain parsers"),
    }
  }

  #[test]
  fn test_choice() {
    let p1 = pure::<char, &'static str>("first");
    let p2 = pure::<char, &'static str>("second");
    let combined = p1.or(p2);
    match combined.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value, .. } => assert_eq!(value, "first"),
      _ => panic!("Or should return first result on success"),
    }
  }

  #[test]
  fn test_and_then() {
    // Test parsing two strings in sequence
    let p1 = pure::<char, &'static str>("hello");
    let p2 = pure::<char, &'static str>("world");

    let combined = p1.and_then(p2);

    match combined.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value: (first, second), .. } => {
        assert_eq!(first, "hello");
        assert_eq!(second, "world");
      }
      _ => panic!("and_then should combine two parsers"),
    }
  }
  
  #[test]
  fn test_skip_left() {
    // Test parsing two strings in sequence (discarding the first result)
    let p1 = pure::<char, &'static str>("hello");
    let p2 = pure::<char, &'static str>("world");

    let combined = p1.skip_left(p2);

    match combined.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value, .. } => {
        assert_eq!(value, "world"); // First result "hello" is discarded
      }
      _ => panic!("skip_left should use only the second parser's result"),
    }
  }

  #[test]
  fn test_and_then_with_error() {
    // Case where the first parser succeeds but the second fails
    let success_parser = pure::<char, &'static str>("ok");

    // Always failing parser
    let failure_parser = move |_: &[char]| -> ParseResult<char, &'static str> { 
      ParseResult::failed_with_uncommitted(ParseError::from(TestError))
    };

    let combined = success_parser.and_then(failure_parser);

    match combined.parse(&"test".chars().collect::<Vec<_>>()) {
      ParseResult::Failure { committed_status, .. } => {
        assert_eq!(committed_status, CommittedStatus::Uncommitted, 
                  "Error from second parser should not be committed");
      }
      _ => panic!("and_then should fail when second parser fails"),
    }
  }
  
  #[test]
  fn test_skip_right() {
    // Test keeping only the first parser's result
    let p1 = pure::<char, &'static str>("hello");
    let p2 = empty::<char>();
    
    let combined = p1.skip_right(p2);
    
    match combined.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value, .. } => {
        assert_eq!(value, "hello");
      }
      _ => panic!("skip_right should keep only the first parser's result"),
    }
  }
  
  #[test]
  fn test_discard() {
    // Test discarding result and returning ()
    let p = pure::<char, &'static str>("hello");
    let discarded = p.discard();
    
    match discarded.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value, .. } => {
        // Result should be ()
        assert_eq!(value, ());
      }
      _ => panic!("discard should return unit value ()"),
    }
    
    // Test combining multiple parsers
    let p1 = pure::<char, &'static str>("hello");
    let p2 = pure::<char, &'static str>("world");
    
    // Discard p1's result and execute p2
    let combined = p1.discard().skip_left(p2);
    
    match combined.parse(&"input".chars().collect::<Vec<_>>()) {
      ParseResult::Success { value, .. } => {
        assert_eq!(value, "world");
      }
      _ => panic!("discard combined with skip_left should return p2's result"),
    }
  }
}
