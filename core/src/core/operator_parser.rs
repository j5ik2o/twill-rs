use crate::core::committed_status::CommittedStatus;
use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;
use crate::core::parser::Parser;
use crate::core::parser_monad::ParserMonad;
use crate::core::{successful, ParseError};

/// Trait providing parser operators
pub trait OperatorParser<'a, I: 'a, A>: Parser<'a, I, A> + ParserMonad<'a, I, A> + Sized {
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
    move |parse_context: ParseContext<'a, I>| match self.parse(parse_context) {
      pr@ParseResult::Failure {
        committed_status: CommittedStatus::Uncommitted,
        ..
      } => {
        let alt = f();
        alt.parse(pr.context().with_same_state())
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
    move |parse_context: ParseContext<'a, I>| match self.parse(parse_context) {
      ParseResult::Success { parse_context, .. } => {
        let len = parse_context.last_offset().unwrap_or(0);
        let parser_error = ParseError::of_mismatch(parse_context, len, "not predicate failed".to_string());
        ParseResult::failed_with_uncommitted(parser_error)
      }
      pr@ParseResult::Failure { .. } => ParseResult::successful(pr.context().with_same_state(), (), 0),
    }
  }

  /// Sequential parser with lazy evaluation (discard first parser result) - implemented using flat_map
  fn skip_left_with<F, P2, B>(self, f: F) -> impl Parser<'a, I, B>
  where
    F: FnOnce() -> P2,
    P2: Parser<'a, I, B>, {
    self.flat_map(move |_| f())
  }

  /// Sequential parser (discard first parser result) - implemented using skip_left_with
  fn skip_left<P2, B>(self, p2: P2) -> impl Parser<'a, I, B>
  where
    P2: Parser<'a, I, B>, {
    self.skip_left_with(move || p2)
  }

  /// Sequential parser with lazy evaluation (discard second parser result) - implemented using flat_map
  fn skip_right_with<F, P2>(self, f: F) -> impl Parser<'a, I, A>
  where
    F: FnOnce() -> P2,
    P2: Parser<'a, I, ()>, {
    self.flat_map(move |a| f().map(move |_| a))
  }

  /// Sequential parser (discard second parser result) - implemented using skip_right_with
  fn skip_right<P2>(self, p2: P2) -> impl Parser<'a, I, A>
  where
    P2: Parser<'a, I, ()>, {
    self.skip_right_with(move || p2)
  }

  /// Discard the result and return ()
  fn discard(self) -> impl Parser<'a, I, ()> {
    self.map(|_| ())
  }

  /// Transforms any failure into an uncommitted failure
  /// This allows the parser to be used in an or_with operation even if it would normally commit
  fn attempt(self) -> impl Parser<'a, I, A> {
    move |parse_context: ParseContext<'a, I>| {
      self.parse(parse_context).with_uncommitted()
    }
  }
  // /// Left associative binary operator parsing with default value
  // ///
  // /// This method takes an operator parser and a default value, and
  // /// returns a parser that repeatedly applies the left associative operation on
  // /// the parsed values, or returns the default value if no operations can be applied.
  fn rest_left1<P2, OP>(self, op: P2, default_value: A) -> impl Parser<'a, I, A>
  where
     P2: Parser<'a, I, OP>,
     OP: FnOnce(A, A) -> A + 'a,
     A: Clone + std::fmt::Debug + 'a, {
    successful(default_value, 0)
  }

}

impl<'a, T, I: 'a, A> OperatorParser<'a, I, A> for T where T: Parser<'a, I, A> + ParserMonad<'a, I, A> {}
