use crate::committed_status::CommittedStatus;
use crate::parse_context::ParseContext;
use crate::parse_error::ParseError;

/// The enum type representing the parse result.
#[derive(Debug, Clone)]
pub enum ParseResult<'a, I, A> {
  /// Success.
  Success {
    /// The parsing context after successful parsing
    parse_context: ParseContext<'a, I>,
    /// The value when success.
    value: A,
    length: usize,
  },
  /// Failure.
  Failure {
    parse_context: ParseContext<'a, I>,
    /// The cause when failure.
    error: ParseError<'a, I>,
    /// The commit status.
    committed_status: CommittedStatus,
  },
}

impl<'a, I, A> ParseResult<'a, I, A> {
  /// Returns the parse result of success.
  ///
  /// - value: a value
  /// - context: the parsing context after successful parsing
  pub fn successful(parse_context: ParseContext<'a, I>, value: A, length: usize) -> Self {
    ParseResult::Success {
      parse_context,
      value,
      length,
    }
  }

  /// Returns the parse result of failure.
  ///
  /// - error: a [ParseError]
  /// - committed_status: a [CommittedStatus]
  pub fn failed(
    parse_context: ParseContext<'a, I>,
    error: ParseError<'a, I>,
    committed_status: CommittedStatus,
  ) -> Self {
    ParseResult::Failure {
      parse_context,
      error,
      committed_status,
    }
  }

  /// Returns the parse result of failure.
  ///
  /// - error: a [ParseError]
  pub fn failed_with_uncommitted(parse_context: ParseContext<'a, I>, error: ParseError<'a, I>) -> Self {
    Self::failed(parse_context, error, CommittedStatus::Uncommitted)
  }

  /// Returns the parse result of failure with committed status.
  ///
  /// - error: a [ParseError]
  pub fn failed_with_commit(parse_context: ParseContext<'a, I>, error: ParseError<'a, I>) -> Self {
    Self::failed(parse_context, error, CommittedStatus::Committed)
  }

  /// Convert [ParseResult] to [Result].
  pub fn to_result(self) -> Result<A, ParseError<'a, I>> {
    match self {
      ParseResult::Failure { error, .. } => Err(error),
      ParseResult::Success { value, .. } => Ok(value),
    }
  }

  pub fn context(self) -> ParseContext<'a, I> {
    match self {
      ParseResult::Failure {
        parse_context, error, ..
      } => parse_context,
      ParseResult::Success { parse_context, .. } => parse_context,
    }
  }

  /// Returns whether the parsing was successful or not.
  pub fn is_success(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => false,
      ParseResult::Success { .. } => true,
    }
  }

  /// Return the results of a successful parsing.
  pub fn success(self) -> Option<A> {
    match self {
      ParseResult::Failure { .. } => None,
      ParseResult::Success { value, .. } => Some(value),
    }
  }

  /// Returns whether the parsing has failed or not.
  pub fn is_failure(&self) -> bool {
    match self {
      ParseResult::Failure { .. } => true,
      ParseResult::Success { .. } => false,
    }
  }

  /// Return the result of the failed parsing.
  pub fn failure(self) -> Option<ParseError<'a, I>> {
    match self {
      ParseResult::Failure { error, .. } => Some(error),
      ParseResult::Success { .. } => None,
    }
  }

  /// Return the committed status.
  pub fn committed_status(&self) -> Option<CommittedStatus> {
    match self {
      ParseResult::Failure {
        committed_status: is_committed,
        ..
      } => Some(*is_committed),
      _ => None,
    }
  }

  /// Unset the commit status when failure
  pub fn with_uncommitted(mut self) -> Self {
    if let ParseResult::Failure { committed_status, .. } = &mut self {
      *committed_status = CommittedStatus::Uncommitted;
    }
    self
  }

  /// Set the commit status with fallback
  pub fn with_committed_fallback(mut self, is_committed: bool) -> Self {
    if let ParseResult::Failure { committed_status, .. } = &mut self {
      *committed_status = committed_status.or(&is_committed.into());
    }
    self
  }

  /// Convert the result to another type (keeping the original failure when failed)
  pub fn flat_map<B, F>(self, f: F) -> ParseResult<'a, I, B>
  where
    F: Fn(ParseContext<'a, I>, A, usize) -> ParseResult<'a, I, B>, {
    match self {
      ParseResult::Success {
        parse_context,
        value,
        length,
      } => f(parse_context, value, length),
      ParseResult::Failure {
        parse_context,
        error: e,
        committed_status: c,
      } => ParseResult::Failure {
        parse_context,
        error: e,
        committed_status: c,
      },
    }
  }

  /// Convert the success value
  pub fn map<B, F>(self, f: F) -> ParseResult<'a, I, B>
  where
    F: Fn(A) -> B, {
    self.flat_map(|parse_context, value, length| {
      let new_value = f(value);
      ParseResult::successful(parse_context, new_value, length)
    })
  }

  /// Convert the error
  pub fn map_err<F>(self, f: F) -> Self
  where
    F: Fn(ParseError<'a, I>) -> ParseError<'a, I>, {
    match self {
      ParseResult::Failure {
        parse_context,
        error: e,
        committed_status: c,
      } => ParseResult::Failure {
        parse_context,
        error: f(e),
        committed_status: c,
      },
      _ => self,
    }
  }

  pub fn with_add_length(self, n: usize) -> Self {
    match self {
      ParseResult::Success {
        parse_context,
        value,
        length,
      } => ParseResult::Success {
        parse_context,
        value,
        length: length + n,
      },
      _ => self,
    }
  }
}
