use crate::parse_context::ParseContext;
use crate::parse_error::ParseError;
use crate::parse_result::ParseResult;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{Parser, RcParser};

/// Trait providing result transformation operations for parsers
pub trait TransformParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + Sized
where
  Self: 'a, {
  /// Discard the result and return ()
  fn discard(self) -> impl Parser<'a, I, ()>
  where
    A: Clone + 'a, {
    self.map(|_| ())
  }

  /// Negation parser - succeeds when self fails, fails when self succeeds
  fn not(self) -> impl Parser<'a, I, ()>
  where
    Self: Sized,
    I: Clone + 'a, {
    RcParser::new(
      move |parse_context: ParseContext<'a, I>| match self.run(parse_context) {
        ParseResult::Success { parse_context, .. } => {
          let len = parse_context.last_offset().unwrap_or(0);
          let parser_error = ParseError::of_mismatch(
            parse_context.input(),
            parse_context.offset(),
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
impl<'a, T, I: 'a, A> TransformParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
