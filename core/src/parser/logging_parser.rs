use crate::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LogLevel {
  Debug,
  Info,
  Warn,
  Err,
}

pub trait LoggingParser<'a, I: 'a, A>: ParserRunner<'a, I, A> + Sized
where
  Self: 'a, {
  fn name(self, name: &'a str) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    A: 'a, {
    Parser::new(move |parse_context| match self.run(parse_context) {
      res @ ParseResult::Success { .. } => res,
      ParseResult::Failure {
        parse_context,
        error,
        committed_status: is_committed,
      } => match error {
        ParseError::Custom { .. } => ParseResult::failed(parse_context, error, is_committed),
        _ => {
          let offset = parse_context.next_offset();
          ParseResult::failed(
            parse_context,
            ParseError::of_custom(offset, Some(Box::new(error)), format!("failed to parse {}", name)),
            is_committed,
          )
        }
      },
    })
  }

  fn expect(self, name: &'a str) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    A: 'a, {
    Parser::new(move |parse_context| match self.run(parse_context.with_same_state()) {
      res @ ParseResult::Success { .. } => res,
      ParseResult::Failure {
        parse_context,
        error,
        committed_status: is_committed,
      } => {
        let offset = parse_context.next_offset();
        ParseResult::failed(
          parse_context,
          ParseError::of_expect(offset, Box::new(error), format!("Expect {}", name)),
          is_committed,
        )
      }
    })
  }

  fn log<B, F>(self, name: &'a str, log_level: LogLevel) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    A: Display + 'a, {
    self.log_map(name, log_level, |pr| format!("{}", pr))
  }

  fn log_map<B, F>(self, name: &'a str, log_level: LogLevel, f: F) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
  where
    A: 'a,
    F: Fn(&ParseResult<'a, I, A>) -> B + 'a,
    B: Display + 'a, {
    Parser::new(move |parse_context| {
      let pr = self.run(parse_context);
      let s = format!("{} = {}", name, f(&pr));
      match log_level {
        LogLevel::Debug => log::debug!("{}", s),
        LogLevel::Info => log::info!("{}", s),
        LogLevel::Warn => log::warn!("{}", s),
        LogLevel::Err => log::error!("{}", s),
      }
      pr
    })
  }
}

impl<'a, T, I: 'a, A: Clone + 'a> LoggingParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}
