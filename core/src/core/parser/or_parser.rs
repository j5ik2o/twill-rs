use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::{ClonableParser, FnParser};

/// Provide alternative parser operations
pub trait OrParser<'a, I: 'a, A>: ClonableParser<'a, I, A>
where
  Self: 'a, {
  /// Try a second parser if the first fails
  fn or<P>(self, other: P) -> impl ClonableParser<'a, I, A>
  where
    A: Clone + 'a,
    P: ClonableParser<'a, I, A> + 'a, {
    self.or_with(move || other.clone())
  }

  /// Try a dynamically generated parser if the first fails
  fn or_with<F, P>(self, f: F) -> impl ClonableParser<'a, I, A>
  where
    A: Clone + 'a,
    P: ClonableParser<'a, I, A> + 'a,
    F: Fn() -> P + Clone + 'a, {
    FnParser::new(
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

/// Add Or methods to all parsers
impl<'a, T, I: 'a, A> OrParser<'a, I, A> for T where T: ClonableParser<'a, I, A> + 'a {}
