use crate::core::parser::rc_parser::to_rc_parser;
use crate::core::parser::rc_parser::to_rc_parser_opt;
use crate::core::util::{Bound, RangeArgument};
use crate::core::{BinaryOperatorParser, ParseContext, ParseError, ParseResult, Parser};
use std::fmt::Debug;

pub trait RepeatParser<'a, I: 'a, A>: Parser<'a, I, A> + BinaryOperatorParser<'a, I, A> + Sized
where
  Self: 'a, {
  fn repeat<R>(self, range: R) -> impl Parser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    A: Clone + Debug + 'a, {
    self.repeat_sep(range, None as Option<Self>)
  }

  fn many0(self) -> impl Parser<'a, I, Vec<A>>
  where
    A: Clone + Debug + 'a, {
    self.repeat_sep(0.., None as Option<Self>)
  }

  fn many1(self) -> impl Parser<'a, I, Vec<A>>
  where
    A: Clone + Debug + 'a, {
    self.repeat_sep(1.., None as Option<Self>)
  }

  fn count<P2, B>(self, count: usize) -> impl Parser<'a, I, Vec<A>>
  where
    P2: Parser<'a, I, B> + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    self.repeat_sep(count..=count, None as Option<P2>)
  }

  fn many0_sep<P2, B>(self, separator: P2) -> impl Parser<'a, I, Vec<A>>
  where
    P2: Parser<'a, I, B> + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    self.repeat_sep(0.., Some(separator))
  }

  fn many1_sep<P2, B>(self, separator: P2) -> impl Parser<'a, I, Vec<A>>
  where
    P2: Parser<'a, I, B> + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    self.repeat_sep(1.., Some(separator))
  }

  fn repeat_sep<P2, B, R>(self, range: R, separator_opt: Option<P2>) -> impl Parser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + Debug + 'a,
    P2: Parser<'a, I, B> + 'a,
    A: Clone + Debug + 'a,
    B: Clone + Debug + 'a, {
    move |pc: ParseContext<'a, I>| {
      let mut all_length = 0;
      let mut items = vec![];

      let rc_parser = to_rc_parser(self);
      let sep_parser_opt = to_rc_parser_opt(separator_opt);

      if let ParseResult::Success {
        parse_context: pc1,
        value,
        length,
      } = rc_parser.clone().parse(pc.with_same_state())
      {
        let mut current_pc = pc1.advance(length);
        items.push(value);
        all_length += length;
        loop {
          match range.end() {
            Bound::Included(&max_count) => {
              if items.len() >= max_count {
                break;
              }
            }
            Bound::Excluded(&max_count) => {
              if items.len() + 1 >= max_count {
                break;
              }
            }
            _ => (),
          }

          if let Some(separator) = &sep_parser_opt {
            if let ParseResult::Success {
              parse_context: pc2,
              length,
              ..
            } = separator.clone().parse(current_pc)
            {
              current_pc = pc2.advance(length);
              all_length += length;
            } else {
              break;
            }
          }

          if let ParseResult::Success {
            parse_context: pc2,
            value,
            length,
          } = rc_parser.clone().parse(current_pc)
          {
            current_pc = pc2.advance(length);
            all_length += length;
            items.push(value);
          } else {
            break;
          }
        }
      }

      if let Bound::Included(&min_count) = range.start() {
        if items.len() < min_count {
          let pc = pc.advance(all_length);
          let pe = ParseError::of_mismatch(
            pc,
            all_length,
            format!("Expected at least {} items, but got {}", min_count, items.len()),
          );
          return ParseResult::failed_with_uncommitted(pe);
        }
      }

      ParseResult::successful(pc, items, all_length)
    }
  }
}
