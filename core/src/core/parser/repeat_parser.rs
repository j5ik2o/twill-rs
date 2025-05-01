use crate::core::parser::rc_parser::to_rc_parser;
use crate::core::parser::rc_parser::to_rc_parser_opt;
use crate::core::parser::FuncParser;
use crate::core::util::{Bound, RangeArgument};
use crate::core::{BinaryOperatorParser, ParseContext, ParseError, ParseResult, Parser};

pub trait RepeatParser<'a, I: 'a, A>: Parser<'a, I, A> + BinaryOperatorParser<'a, I, A> + Sized
where
  Self: 'a, {
  fn repeat<R>(self, range: R) -> impl Parser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + 'a,
    A: 'a, {
    self.repeat_sep(range, None as Option<Self>)
  }

  fn many0(self) -> impl Parser<'a, I, Vec<A>>
  where
    A: 'a, {
    self.repeat_sep(0.., None as Option<Self>)
  }

  fn many1(self) -> impl Parser<'a, I, Vec<A>>
  where
    A: 'a, {
    self.repeat_sep(1.., None as Option<Self>)
  }

  fn count(self, count: usize) -> impl Parser<'a, I, Vec<A>>
  where
    A: 'a, {
    self.repeat_sep(count..=count, None as Option<Self>)
  }

  fn many0_sep<P2, B>(self, separator: P2) -> impl Parser<'a, I, Vec<A>>
  where
    P2: Parser<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    self.repeat_sep(0.., Some(separator))
  }

  fn many1_sep<P2, B>(self, separator: P2) -> impl Parser<'a, I, Vec<A>>
  where
    P2: Parser<'a, I, B> + 'a,
    A: 'a,
    B: 'a, {
    self.repeat_sep(1.., Some(separator))
  }

  fn repeat_sep<P2, B, R>(self, range: R, separator_opt: Option<P2>) -> impl Parser<'a, I, Vec<A>>
  where
    R: RangeArgument<usize> + 'a,
    A: 'a,
    B: 'a,
    P2: Parser<'a, I, B> + 'a, {
    let parser = to_rc_parser(self);
    let separator = to_rc_parser_opt(separator_opt);
    FuncParser::new(move |parse_context| {
      let mut all_length = 0;
      let mut items = vec![];

      if let ParseResult::Success {
        parse_context: pc1,
        value,
        length,
      } = parser.clone().run(parse_context.with_same_state())
      {
        let mut current_parse_state = pc1.advance(length);
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

          if let Some(sep) = &separator {
            if let ParseResult::Success {
              parse_context: pc2,
              length,
              ..
            } = sep.clone().run(current_parse_state)
            {
              current_parse_state = pc2.advance(length);
              all_length += length;
            } else {
              break;
            }
          }
          if let ParseResult::Success {
            parse_context: pc3,
            value,
            length,
          } = parser.clone().run(current_parse_state)
          {
            current_parse_state = pc3.advance(length);
            items.push(value);
            all_length += length;
          } else {
            break;
          }
        }
      }

      if let Bound::Included(&min_count) = range.start() {
        if items.len() < min_count {
          let pc = parse_context.advance(all_length);
          let pe = ParseError::of_mismatch(
            pc,
            all_length,
            format!(
              "expect repeat at least {} times, found {} times",
              min_count,
              items.len()
            ),
          );
          return ParseResult::failed_with_uncommitted(pe);
        }
      }
      ParseResult::successful(parse_context, items, all_length)
    })
  }
}
