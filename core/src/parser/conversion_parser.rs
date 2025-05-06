use crate::prelude::*;
use std::fmt::Debug;

pub trait ConversionParser<'a, I: 'a, A>: Parser<'a, I, A> + Sized {
  fn map_res<B, E, F>(self, f: F) -> RcParser<'a, I, B, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: 'a,
    B: 'a, {
    RcParser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        parse_context,
        value: a,
        length,
      } => match f(a) {
        Ok(value) => ParseResult::successful(parse_context, value, length),
        Err(err) => {
          let msg = format!("Conversion error: {:?}", err);
          let input = parse_context.input();
          let offset = parse_context.offset();
          let parser_error = ParseError::of_conversion(input, offset, 0, msg);
          ParseResult::failed_with_uncommitted(parse_context, parser_error)
        }
      },
      ParseResult::Failure {
        parse_context,
        error,
        committed_status,
      } => ParseResult::failed(parse_context, error, committed_status),
    })
  }

  fn map_opt<B, E, F>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: Fn(A) -> Option<B> + 'a,
    A: 'a,
    B: 'a, {
    RcParser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        parse_context,
        value: a,
        length,
      } => match f(a) {
        Some(value) => ParseResult::successful(parse_context, value, length),
        None => {
          let input = parse_context.input();
          let offset = parse_context.offset();
          let parser_error = ParseError::of_conversion(input, offset, 0, "Conversion error".to_string());
          ParseResult::failed_with_uncommitted(parse_context, parser_error)
        }
      },
      ParseResult::Failure {
        parse_context,
        error,
        committed_status,
      } => ParseResult::failed(parse_context, error, committed_status),
    })
  }
}

impl<'a, T, I: 'a, A: Clone + 'a> ConversionParser<'a, I, A> for T where T: Parser<'a, I, A> + 'a {}
