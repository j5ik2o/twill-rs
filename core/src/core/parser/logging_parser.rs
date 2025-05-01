use crate::core::parser::FuncParser;
use crate::core::{ParseError, ParseResult, Parser};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LogLevel {
  Debug,
  Info,
  Warn,
  Err,
}

pub trait LoggingParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized
where
  Self: 'a, {
  fn name(self, name: &'a str) -> impl Parser<'a, I, A> where A: Clone + 'a{
    FuncParser::new(move |parse_context| match self.clone().run(parse_context.with_same_state()) {
      res @ ParseResult::Success { .. } => res,
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => match error {
        ParseError::Custom { .. } => ParseResult::failed(error, is_committed),
        _ => ParseResult::failed(
          ParseError::of_custom(
            parse_context,
            Some(Box::new(error)),
            format!("failed to parse {}", name),
          ),
          is_committed,
        ),
      },
    })
  }

  fn expect(self, name: &'a str) -> impl Parser<'a, I, A>where A: Clone + 'a {
    FuncParser::new(move |parse_context| match self.clone().run(parse_context.with_same_state()) {
      res @ ParseResult::Success { .. } => res,
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(
        ParseError::of_expect(parse_context, Box::new(error), format!("Expect {}", name)),
        is_committed,
      ),
    })
  }

  fn log_map<B, F>(self, name: &'a str, log_level: LogLevel, f: F) -> impl Parser<'a, I, A>
  where A: Clone + 'a,
    F: Fn(&ParseResult<'a, I, A>) -> B + Clone + 'a,
    B: Display + 'a, {
    FuncParser::new(move |parse_context| {
      let pr = self.clone().run(parse_context);
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

impl<'a, T, I: 'a, A: Clone + 'a> LoggingParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
