use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::{FuncParser, Parser};

/// Provide alternative parser operations
pub trait OrParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized + Clone
where
  Self: 'a, {
  /// Try a second parser if the first fails
  fn or<P>(self, other: P) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    P: Parser<'a, I, A> + Clone + 'a, {
    self.or_with(move || other.clone())
  }

  /// Try a dynamically generated parser if the first fails
  fn or_with<F, P>(self, f: F) -> impl Parser<'a, I, A>
  where
    A: Clone + 'a,
    P: Parser<'a, I, A> + 'a,
    F: Fn() -> P + Clone + 'a, {
    FuncParser::new(
      move |parse_context: ParseContext<'a, I>| match self.clone().run(parse_context) {
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
impl<'a, T, I: 'a, A> OrParser<'a, I, A> for T where T: Parser<'a, I, A> + Clone + 'a {}
