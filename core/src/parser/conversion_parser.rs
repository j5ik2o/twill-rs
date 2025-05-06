use crate::prelude::*;
use std::fmt::Debug;

pub trait ConversionParser<'a, I: 'a, A>: ParserRunner<'a, I, A> + Sized {
  fn map_res<B, E, F>(self, f: F) -> Parser<'a, I, B, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a>
  where
      I: Debug ,
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        parse_context,
        value: a,
        length,
      } => match f(a) {
        Ok(value) => ParseResult::successful(parse_context, value, length),
        Err(err) => {
          let pc = parse_context.add_offset(0);
          let msg = format!("Conversion error: {:?}", err);
          let input = pc.input();
          let offset = pc.last_offset().unwrap_or(0);
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

  fn map_opt<B, E, F>(self, f: F) -> Parser<'a, I, B, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a>
  where
      I: Debug,
    F: Fn(A) -> Option<B> + 'a,
    A: 'a,
    B: 'a, {
    Parser::new(move |parse_context| match self.run(parse_context) {
      ParseResult::Success {
        parse_context,
        value: a,
        length,
      } => match f(a) {
        Some(value) => ParseResult::successful(parse_context, value, length),
        None => {
          let pc = parse_context.with_same_state();
          let msg = "Conversion error".to_string();
          let input = pc.input();
          let offset = pc.last_offset().unwrap_or(0);
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
}

impl<'a, T, I: 'a, A: Clone + 'a> ConversionParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}
