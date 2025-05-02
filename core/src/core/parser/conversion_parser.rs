use crate::core::{ClonableParser, FnParser, ParseError, ParseResult};
use std::fmt::Debug;

pub trait ConversionParser<'a, I: 'a, A>: ClonableParser<'a, I, A> + Sized {
  fn map_res<B, E, F>(self, f: F) -> impl ClonableParser<'a, I, B>
  where
    Self: Clone + 'a,
    F: Fn(A) -> Result<B, E> + Clone + 'a,
    E: Debug,
    A: 'a,
    B: Clone + 'a, {
    FnParser::new(move |parse_context| match self.clone().run(parse_context) {
      ParseResult::Success {
        parse_context,
        value: a,
        length,
      } => match f(a) {
        Ok(value) => ParseResult::successful(parse_context, value, length),
        Err(err) => {
          let msg = format!("Conversion error: {:?}", err);
          let parser_error = ParseError::of_conversion(parse_context, 0, msg);
          ParseResult::failed_with_uncommitted(parser_error)
        }
      },
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }

  fn map_opt<B, E, F>(self, f: F) -> impl ClonableParser<'a, I, B>
  where
    F: Fn(A) -> Option<B> + Clone + 'a,
    A: 'a,
    B: Clone + 'a, {
    FnParser::new(move |parse_context| match self.clone().run(parse_context) {
      ParseResult::Success {
        parse_context,
        value: a,
        length,
      } => match f(a) {
        Some(value) => ParseResult::successful(parse_context, value, length),
        None => {
          let msg = format!("Conversion error");
          let parser_error = ParseError::of_conversion(parse_context, 0, msg);
          ParseResult::failed_with_uncommitted(parser_error)
        }
      },
      ParseResult::Failure {
        error,
        committed_status: is_committed,
      } => ParseResult::failed(error, is_committed),
    })
  }
}

impl<'a, T, I: 'a, A: Clone + 'a> ConversionParser<'a, I, A> for T where T: ClonableParser<'a, I, A> + 'a {}
