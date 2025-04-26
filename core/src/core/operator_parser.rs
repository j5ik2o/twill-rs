use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;

/// Trait providing parser operators
pub trait OperatorParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized {
  /// Apply parsers selectively (disjunction)
  fn or<P>(self, alt: P) -> impl Parser<'a, I, A>
  where
    P: Parser<'a, I, A>, {
    move |input: &ParseContext<'a, I>| match self.parse(input) {
      failure @ ParseResult::Failure {
        committed_status: CommittedStatus::Committed,
        ..
      } => failure,
      ParseResult::Failure {
        committed_status: CommittedStatus::Uncommitted,
        ..
      } => alt.parse(input),
      success @ ParseResult::Success { .. } => success,
    }
  }

  /// Sequential parser (conjunction) - standard version (requires Clone)
  fn and_then<P2, B>(self, p2: P2) -> impl Parser<'a, I, (A, B)>
  where
    A: Clone,
    P2: Parser<'a, I, B>, {
    move |input: &ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Success { value: v1, context } => match p2.parse(&context) {
        ParseResult::Success {
          value: v2,
          context: new_context,
        } => ParseResult::successful((v1.clone(), v2), new_context),
        ParseResult::Failure {
          error,
          committed_status,
        } => ParseResult::failed(error, committed_status),
      },
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }

  /// Sequential parser (discard first parser result) - avoid Clone
  fn skip_left<P2, B>(self, p2: P2) -> impl Parser<'a, I, B>
  where
    P2: Parser<'a, I, B>, {
    move |input: &ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Success { context, .. } => p2.parse(&context),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }

  /// Sequential parser (discard second parser result) - avoid Clone
  fn skip_right<P2>(self, p2: P2) -> impl Parser<'a, I, A>
  where
    A: Clone,
    P2: Parser<'a, I, ()>, {
    move |input: &ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Success { value: v1, context } => match p2.parse(&context) {
        ParseResult::Success {
          context: new_context, ..
        } => ParseResult::successful(v1.clone(), new_context),
        ParseResult::Failure {
          error,
          committed_status,
        } => ParseResult::failed(error, committed_status),
      },
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }

  /// Discard result and return ()
  fn discard(self) -> impl Parser<'a, I, ()> {
    move |input: &ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Success { context, .. } => ParseResult::successful((), context),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }
}

/// Provide operator methods to all parsers
impl<'a, T, I: 'a, A> OperatorParser<'a, I, A> for T where T: Parser<'a, I, A> {}
