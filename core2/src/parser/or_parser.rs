use crate::parse_context::ParseContext;
use crate::parser::{Parser, RcParser};

/// Provide alternative parser operations
pub trait OrParser<'a, I: 'a, A>: Parser<'a, I, A>
where
  Self: 'a, {
  /// Try a dynamically generated parser if the first fails
  fn or<P>(self, other: P) -> impl Parser<'a, I, A>
  where
    A: 'a,
    P: Parser<'a, I, A> + 'a, {
    RcParser::new(move |parse_context: ParseContext<'a, I>| {
      let result = self.run(parse_context.with_same_state());
      if let Some(committed_status) = result.committed_status() {
        if committed_status.is_uncommitted() {
          return other.run(parse_context);
        }
      }
      result
    })
  }
}

/// Add Or methods to all parsers
impl<'a, T, I: 'a, A> OrParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  fn test_or() {
    {
      let text: &str = "a";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_ref('a').or(elm_ref('b'));

      let result = p.parse(&input).to_result();
      println!("{:?}", result);

      assert!(result.is_ok());
    }

    {
      let text: &str = "b";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_ref('a').or(elm_ref('b'));

      let result = p.parse(&input).to_result();
      println!("{:?}", result);

      assert!(result.is_ok());
    }
  }
}
