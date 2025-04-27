use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;
use crate::core::parser_ext::ParserExt;
use crate::core::ParseError;

/// Trait providing parser operators
pub trait OperatorParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserExt<'a, I, A> + Sized {
  /// Apply parsers selectively (disjunction) with lazy alternative evaluation
  fn or<P>(self, alt: P) -> impl Parser<'a, I, A>
  where
    P: Parser<'a, I, A>, {
    self.or_with(|| alt)
  }

  fn or_with<F, P>(self, f: F) -> impl Parser<'a, I, A>
  where
    F: FnOnce() -> P,
    P: Parser<'a, I, A>, {
    move |input: &ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Failure {
        committed_status: CommittedStatus::Uncommitted,
        ..
      } => {
        let alt = f();
        alt.parse(input)
      }
      other => other,
    }
  }

  /// Sequential parser that uses a function to create the second parser
  fn and_then_with<F, P2, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: FnOnce(A) -> P2,
    P2: Parser<'a, I, B>, {
    self.flat_map(f)
  }

  /// Sequential parser (conjunction) - implemented using flat_map and map (no Clone required)
  fn and_then<P2, B>(self, p2: P2) -> impl Parser<'a, I, (A, B)>
  where
    P2: Parser<'a, I, B>, {
    self.and_then_with(move |a| p2.map(move |b| (a, b)))
  }

  /// Negation parser - succeeds when self fails, fails when self succeeds
  fn not(self) -> impl Parser<'a, I, ()> {
    move |input: &ParseContext<'a, I>| match self.parse(input) {
      ParseResult::Success { context, .. } => {
        let parser_error = ParseError::of_mismatch(context, 0, "not predicate failed".to_string());
        ParseResult::failed_with_uncommitted(parser_error)
      }
      ParseResult::Failure { .. } => ParseResult::successful((), input.with_same_state()),
    }
  }

  /// Sequential parser (discard first parser result) - implemented using flat_map
  fn skip_left<P2, B>(self, p2: P2) -> impl Parser<'a, I, B>
  where
    P2: Parser<'a, I, B>, {
    self.flat_map(move |_| p2)
  }

  /// Sequential parser (discard second parser result) - implemented using flat_map (no Clone required)
  fn skip_right<P2>(self, p2: P2) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, ()>, {
    self.flat_map(move |a| p2.map(move |_| a))
  }

  /// Discard the result and return ()
  fn discard(self) -> impl Parser<'a, I, ()> {
    self.map(|_| ())
  }
}

/// Provide operator methods to all parsers
impl<'a, T, I: 'a, A> OperatorParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserExt<'a, I, A> {}
