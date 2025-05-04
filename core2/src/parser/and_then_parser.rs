use crate::parse_result::ParseResult;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{Parser, RcParser}; // successful は不要に // ParseResult を直接使う

/// Trait providing sequence-related parser operations (consuming self)
pub trait AndThenParser<'a, I: 'a, A>: ParserMonad<'a, I, A> {
  /// Sequential parser (conjunction) - implemented directly using RcParser (consuming self)
  fn and_then<P2, B>(self, p2: &'a P2) -> impl Parser<'a, I, (A, B)>
  // Changed to take self
  where
    A: 'a,
    B: 'a,
    P2: Parser<'a, I, B> + 'a, {
    RcParser::new(move |parse_context1| match self.run(parse_context1) {
      ParseResult::Success {
        parse_context: parse_context2,
        value: a,
        length: length1,
      } => match p2.run(parse_context2.advance(length1)) {
        ParseResult::Success {
          parse_context: parse_context3,
          value: b,
          length: length2,
        } => ParseResult::successful(parse_context3, (a, b), length1 + length2),
        ParseResult::Failure {
          parse_context: parse_context3,
          error,
          committed_status,
        } => ParseResult::failed(parse_context3, error, committed_status),
      },
      ParseResult::Failure {
        parse_context: parse_context2,
        error,
        committed_status,
      } => ParseResult::failed(parse_context2, error, committed_status),
    })
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> AndThenParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}
