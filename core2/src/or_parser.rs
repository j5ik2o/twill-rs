use crate::committed_status::CommittedStatus;
use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, RcParser};

/// Provide alternative parser operations
pub trait OrParser<'a, I: 'a, A>: Parser<'a, I, A>
where
  Self: 'a, {
  /// Try a second parser if the first fails
  fn or<P>(self, other: &'a P) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    P: Parser<'a, I, A> + Clone + 'a, {
    self.or_with(move || other)
  }

  /// Try a dynamically generated parser if the first fails
  fn or_with<F, P>(self, f: F) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    P: Parser<'a, I, A> + 'a,
    F: Fn() -> &'a P + Clone + 'a, {
    RcParser::new(
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
impl<'a, T, I: 'a, A> OrParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
