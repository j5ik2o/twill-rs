use std::ops::BitOr;
use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, ParserRunner};

/// Provide alternative parser operations
pub trait OrParser<'a, I: 'a, A>: ParserRunner<'a, I, A>
where
  Self: 'a, {
  /// Try a dynamically generated parser if the first fails
  fn or<P>(self, other: P) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    A: 'a,
    P: ParserRunner<'a, I, A> + 'a, {
    Parser::new(move |parse_context: ParseContext<'a, I>| {
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

impl<'a, I, A, F, G> BitOr<Parser<'a, I, A, F>> for Parser<'a, I, A, G>
where
    A: Clone + 'a,
    F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
    G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
{
  type Output = Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>;

  fn bitor(self, rhs: Parser<'a, I, A, F>) -> Self::Output {
    self.or(rhs)
  }
}

/// Add Or methods to all parsers
impl<'a, T, I: 'a, A> OrParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}

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

  #[test]
  fn test_or_2() {
    {
      let text: &str = "a";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_ref('a') | (elm_ref('b'));

      let result = p.parse(&input).to_result();
      println!("{:?}", result);

      assert!(result.is_ok());
    }

    {
      let text: &str = "b";
      let input = text.chars().collect::<Vec<_>>();
      let p = elm_ref('a') | (elm_ref('b'));

      let result = p.parse(&input).to_result();
      println!("{:?}", result);

      assert!(result.is_ok());
    }
  }
}
