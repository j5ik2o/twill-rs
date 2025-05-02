use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::{ClonableParser, FnParser};
use crate::core::ParseError;

/// Trait providing result transformation operations for parsers
pub trait TransformParser<'a, I: 'a, A>: ClonableParser<'a, I, A> + ParserMonad<'a, I, A> + Sized
where
  Self: 'a, {
  /// Discard the result and return ()
  fn discard(self) -> impl ClonableParser<'a, I, ()>
  where
    A: Clone + 'a, {
    self.map(|_| ())
  }

  /// Negation parser - succeeds when self fails, fails when self succeeds
  fn not(self) -> impl ClonableParser<'a, I, ()> {
    FnParser::new(
      move |parse_context: ParseContext<'a, I>| match self.clone().run(parse_context) {
        ParseResult::Success { parse_context, .. } => {
          let len = parse_context.last_offset().unwrap_or(0);
          let parser_error = ParseError::of_mismatch(parse_context, len, "not predicate failed".to_string());
          ParseResult::failed_with_uncommitted(parser_error)
        }
        pr @ ParseResult::Failure { .. } => ParseResult::successful(pr.context().with_same_state(), (), 0),
      },
    )
  }
}

/// Implement TransformParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> TransformParser<'a, I, A> for T where T: ClonableParser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
