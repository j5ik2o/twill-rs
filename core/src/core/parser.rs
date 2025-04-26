use crate::core::committed_status::CommittedStatus;
use crate::core::parse_result::ParseResult;

/// Basic parser trait
pub trait Parser<'a, I: 'a, A> {
  fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A>;
}

/// Treat closures as parsers
impl<'a, F, I, A> Parser<'a, I, A> for F
where
  F: Fn(&'a [I]) -> ParseResult<'a, I, A>,
  I: 'a,
{
  fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, A> {
    self(input)
  }
}

/// Trait providing parser operators
pub trait OperatorParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized {
  /// Apply parsers selectively (disjunction)
  fn or<P>(self, alt: P) -> impl Parser<'a, I, A>
  where
    P: Parser<'a, I, A>, {
    move |input: &'a [I]| match self.parse(input) {
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
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { value: v1, length: l1 } => {
        let remaining = &input[l1..];
        match p2.parse(remaining) {
          ParseResult::Success { value: v2, length: l2 } => ParseResult::successful((v1.clone(), v2), l1 + l2),
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status),
        }
      }
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
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { length: l1, .. } => {
        let remaining = &input[l1..];
        p2.parse(remaining).map(|value, l2| (value, l1 + l2))
      }
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
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { value: v1, length: l1 } => {
        let remaining = &input[l1..];
        match p2.parse(remaining) {
          ParseResult::Success { length: l2, .. } => ParseResult::successful(v1.clone(), l1 + l2),
          ParseResult::Failure {
            error,
            committed_status,
          } => ParseResult::failed(error, committed_status),
        }
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }

  /// Discard result and return ()
  fn discard(self) -> impl Parser<'a, I, ()> {
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { length, .. } => ParseResult::successful((), length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }
}

/// Trait providing parser transformation methods
pub trait ParserExt<'a, I: 'a, A>: Parser<'a, I, A> + Sized {
  /// Transform success result
  fn map<F, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: Fn(A) -> B, {
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { value, length } => ParseResult::successful(f(value), length),
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }

  /// Chain parsers
  fn flat_map<F, P, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: Fn(A) -> P,
    P: Parser<'a, I, B>, {
    move |input: &'a [I]| match self.parse(input) {
      ParseResult::Success { value, length } => {
        let remaining = &input[length..];
        f(value).parse(remaining).with_add_length(length)
      }
      ParseResult::Failure {
        error,
        committed_status,
      } => ParseResult::failed(error, committed_status),
    }
  }
}

/// Provide extension methods to all parsers
impl<'a, T, I: 'a, A> ParserExt<'a, I, A> for T where T: Parser<'a, I, A> {}

/// Provide operator methods to all parsers
impl<'a, T, I: 'a, A> OperatorParser<'a, I, A> for T where T: Parser<'a, I, A> {}

/// Always successful parser
pub fn pure<'a, I: 'a, A: Clone>(value: A) -> impl Parser<'a, I, A> {
  let value = value.clone();
  move |_input: &'a [I]| ParseResult::successful(value.clone(), 0)
}

/// Do nothing parser - does not consume input and returns no value
pub fn empty<'a, I: 'a>() -> impl Parser<'a, I, ()> {
  move |_input: &'a [I]| ParseResult::successful((), 0)
}
