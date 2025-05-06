use std::ops::Add;
use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::parser_monad::ParserMonad;
use crate::parser::{Parser, RcParser};

/// Trait providing sequence-related parser operations (consuming self)
pub trait AndThenParser<'a, I: 'a, A>: ParserMonad<'a, I, A> {
  /// Sequential parser (conjunction) - implemented directly using RcParser (consuming self)
  fn and_then<P2, B>(
    self,
    p2: P2,
  ) -> RcParser<'a, I, (A, B), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, (A, B)> + 'a>
  where
    A: Clone + 'a,
    B: Clone + 'a,
    P2: Parser<'a, I, B> + Clone + 'a, {
    self.clone().flat_map(move |a| p2.clone().map(move |b| (a.clone(), b)))
  //   RcParser::new(move |parse_context1| match self.run(parse_context1) {
  //     ParseResult::Success {
  //       parse_context: parse_context2,
  //       value: a,
  //       length: length1,
  //     } => match p2.run(parse_context2.advance(length1)) {
  //       ParseResult::Success {
  //         parse_context: parse_context3,
  //         value: b,
  //         ..
  //       } => ParseResult::successful(parse_context3, (a, b), length1),
  //       ParseResult::Failure {
  //         parse_context: parse_context3,
  //         error,
  //         committed_status,
  //       } => ParseResult::failed(parse_context3, error, committed_status),
  //     },
  //     ParseResult::Failure {
  //       parse_context: parse_context2,
  //       error,
  //       committed_status,
  //     } => ParseResult::failed(parse_context2, error, committed_status),
  //   })
  }
}

/// Implement SequenceParser for all types that implement Parser and ParserMonad
impl<'a, T, I: 'a, A> AndThenParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> + 'a {}

impl<'a, I, A, F, B, G> Add<RcParser<'a, I, B, G>> for RcParser<'a, I, A, F>
  where
    A: Clone + 'a,
    B: Clone + 'a,
    F: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a,
    G: Fn(ParseContext<'a, I>) -> ParseResult<'a, I, B> + 'a,
{
  type Output = RcParser<'a, I, (A, B), impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, (A, B)> + 'a>;

  fn add(self, rhs: RcParser<'a, I, B, G>) -> Self::Output {
    self.and_then(rhs)
  }
}