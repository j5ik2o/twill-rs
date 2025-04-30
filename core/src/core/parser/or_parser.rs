use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::{FuncParser, Parser};

/// Trait providing choice-related parser operations
pub trait OrParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized
where
  Self: 'a, {
  /// Apply parsers selectively (disjunction) with lazy alternative evaluation
  fn or<P>(self, alt: P) -> impl Parser<'a, I, A>
  where
    P: Parser<'a, I, A> + 'a, {
    self.or_with(|| alt)
  }

  /// Apply parsers selectively (disjunction) with lazy alternative evaluation using a function
  fn or_with<F, P>(self, f: F) -> impl Parser<'a, I, A>
  where
    P: Parser<'a, I, A> + 'a,
    F: FnOnce() -> P + 'a, {
    FuncParser::new(
      move |parse_context: ParseContext<'a, I>| match self.run(parse_context) {
        pr @ ParseResult::Failure {
          committed_status: CommittedStatus::Uncommitted,
          ..
        } => {
          let alt = f();
          alt.run(pr.context().with_same_state())
        }
        other => other,
      },
    )
  }
}

/// Implement ChoiceParser for all types that implement Parser
impl<'a, T, I: 'a, A> OrParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
