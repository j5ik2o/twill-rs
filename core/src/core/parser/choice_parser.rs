use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// Trait providing choice-related parser operations
pub trait ChoiceParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized
where
  Self: 'a, {
  /// Apply parsers selectively (disjunction) with lazy alternative evaluation
  fn or<P>(self, alt: P) -> impl Parser<'a, I, A>
  where
    P: Parser<'a, I, A>, {
    self.or_with(|| alt)
  }

  /// Apply parsers selectively (disjunction) with lazy alternative evaluation using a function
  fn or_with<F, P>(self, f: F) -> impl Parser<'a, I, A>
  where
    F: FnOnce() -> P,
    P: Parser<'a, I, A>, {
    move |parse_context: ParseContext<'a, I>| match self.parse(parse_context) {
      pr @ ParseResult::Failure {
        committed_status: CommittedStatus::Uncommitted,
        ..
      } => {
        let alt = f();
        alt.parse(pr.context().with_same_state())
      }
      other => other,
    }
  }

  /// Transforms any failure into an uncommitted failure
  /// This allows the parser to be used in an or_with operation even if it would normally commit
  fn attempt(self) -> impl Parser<'a, I, A> {
    move |parse_context: ParseContext<'a, I>| self.parse(parse_context).with_uncommitted()
  }
}

/// Implement ChoiceParser for all types that implement Parser
impl<'a, T, I: 'a, A> ChoiceParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
