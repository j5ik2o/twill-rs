use std::fmt::Debug;
use crate::parse_context::ParseContext;
use crate::parse_error::ParseError;
use crate::parse_result::ParseResult;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{ParserRunner, Parser};

/// Trait providing result transformation operations for parsers
pub trait TransformParser<'a, I: 'a, A>: ParserRunner<'a, I, A> + ParserMonad<'a, I, A> + Sized
where
  Self: 'a, {
  /// Discard the result and return ()
  fn discard(self) -> Parser<'a, I, (), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, ()> + 'a>
  where
    A: Clone + 'a, {
    self.map(|_| ())
  }

  /// Negation parser - succeeds when self fails, fails when self succeeds
  fn not(self) -> Parser<'a, I, (), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, ()> + 'a>
  where
    Self: Sized,
    I: Debug + 'a, {
    Parser::new(
      move |parse_context: ParseContext<'a, I>| match self.run(parse_context) {
        ParseResult::Success { parse_context, .. } => {
          let len = parse_context.last_offset().unwrap_or(0);
          let parser_error = ParseError::of_mismatch(
            parse_context.input(),
            parse_context.next_offset(),
            len,
            "not predicate failed".to_string(),
          );
          ParseResult::failed_with_uncommitted(parse_context, parser_error)
        }
        pr @ ParseResult::Failure { .. } => ParseResult::successful(pr.context().with_same_state(), (), 0),
      },
    )
  }
}

/// Implement TransformParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> TransformParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
