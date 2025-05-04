use crate::core::parse_context::ParseContext;
use std::fmt;
use std::fmt::Display;

/// The enum type representing the parsing error.
#[derive(Debug, Clone)]
pub enum ParseError<'a, I> {
  /// Error when the parser's condition does not match
  Mismatch {
    parse_context: ParseContext<'a, I>,
    length: usize,
    message: String,
  },
  /// Error when conversion fails
  Conversion {
    parse_context: ParseContext<'a, I>,
    length: usize,
    message: String,
  },
  /// Error when parsing is interrupted
  Incomplete { parse_context: ParseContext<'a, I> },
  /// Error when the result deviates from expectations
  Expect {
    parse_context: ParseContext<'a, I>,
    inner: Box<ParseError<'a, I>>,
    message: String,
  },
  /// Custom error
  Custom {
    parse_context: ParseContext<'a, I>,
    inner: Option<Box<ParseError<'a, I>>>,
    message: String,
  },
}

impl<I> Display for ParseError<'_, I> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ParseError::Incomplete { .. } => write!(f, "Incomplete"),
      ParseError::Mismatch {
        ref message,
        parse_context,
        ..
      } => write!(f, "Mismatch at {}: {}", parse_context.offset(), message),
      ParseError::Conversion {
        ref message,
        parse_context,
        ..
      } => write!(f, "Conversion failed at {}: {}", parse_context.offset(), message),
      ParseError::Expect {
        parse_context,
        ref message,
        ref inner,
        ..
      } => write!(f, "{} at {}: {}", message, parse_context.offset(), inner),
      ParseError::Custom {
        parse_context,
        ref message,
        inner: Some(ref inner),
        ..
      } => write!(f, "{} at {}, (inner: {})", message, parse_context.offset(), inner),
      ParseError::Custom {
        parse_context,
        ref message,
        inner: None,
        ..
      } => write!(f, "{} at {}", message, parse_context.offset()),
    }
  }
}

impl ParseError<'_, char> {
  pub fn input_string(&self) -> Option<String> {
    self.input().map(|chars| String::from_iter(chars))
  }
}

impl ParseError<'_, u8> {
  pub fn input_string(&self) -> Option<String> {
    match self.input() {
      Some(bytes) => match std::str::from_utf8(bytes) {
        Ok(s) => Some(s.to_string()),
        Err(_) => Some("".to_string()),
      },
      None => None,
    }
  }
}

impl<'a, I> ParseError<'a, I> {
  pub fn input(&self) -> Option<&'a [I]> {
    match self {
      ParseError::Incomplete { .. } => None,
      ParseError::Mismatch {
        parse_context: context,
        length,
        ..
      } => Some(context.slice_with_len(*length)),
      ParseError::Conversion {
        parse_context: context,
        length,
        ..
      } => Some(context.slice_with_len(*length)),
      ParseError::Expect { ref inner, .. } => inner.input(),
      ParseError::Custom {
        inner: Some(ref inner), ..
      } => inner.input(),
      ParseError::Custom { inner: None, .. } => None,
    }
  }

  pub fn offset(&self) -> Option<usize> {
    match self {
      ParseError::Incomplete { .. } => None,
      ParseError::Mismatch { parse_context, .. } => Some(parse_context.offset()),
      ParseError::Conversion { parse_context, .. } => Some(parse_context.offset()),
      ParseError::Expect { parse_context, .. } => Some(parse_context.offset()),
      ParseError::Custom { parse_context, .. } => Some(parse_context.offset()),
    }
  }

  pub fn is_expect(&self) -> bool {
    matches!(self, ParseError::Expect { .. })
  }

  pub fn is_custom(&self) -> bool {
    matches!(self, ParseError::Custom { .. })
  }

  pub fn is_mismatch(&self) -> bool {
    matches!(self, ParseError::Mismatch { .. })
  }

  pub fn is_conversion(&self) -> bool {
    matches!(self, ParseError::Conversion { .. })
  }

  pub fn is_in_complete(&self) -> bool {
    matches!(self, ParseError::Incomplete { .. })
  }

  pub fn parse_context(&self) -> &ParseContext<'a, I> {
    match self {
      ParseError::Incomplete { parse_context: context } => context,
      ParseError::Mismatch {
        parse_context: context, ..
      } => context,
      ParseError::Conversion {
        parse_context: context, ..
      } => context,
      ParseError::Expect {
        parse_context: context, ..
      } => context,
      ParseError::Custom {
        parse_context: context, ..
      } => context,
    }
  }

  pub fn of_expect(context: ParseContext<'a, I>, inner: Box<ParseError<'a, I>>, message: String) -> Self {
    ParseError::Expect {
      parse_context: context,
      inner,
      message,
    }
  }

  pub fn of_custom(context: ParseContext<'a, I>, inner: Option<Box<ParseError<'a, I>>>, message: String) -> Self {
    ParseError::Custom {
      parse_context: context,
      inner,
      message,
    }
  }

  pub fn of_mismatch(context: ParseContext<'a, I>, length: usize, message: String) -> Self {
    ParseError::Mismatch {
      parse_context: context,
      length,
      message,
    }
  }

  pub fn of_conversion(context: ParseContext<'a, I>, length: usize, message: String) -> Self {
    ParseError::Conversion {
      parse_context: context,
      length,
      message,
    }
  }

  pub fn of_in_complete(context: ParseContext<'a, I>) -> Self {
    ParseError::Incomplete { parse_context: context }
  }
}
