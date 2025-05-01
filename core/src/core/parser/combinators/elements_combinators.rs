use crate::core::element::Element;
use crate::core::parser::FuncParser;
use crate::core::util::Set;
use crate::core::{ParseContext, ParseError, ParseResult, Parser};
use std::fmt::Display;

pub fn elm_pred_ref<'a, I: 'a, F>(f: F) -> impl Parser<'a, I, &'a I>
where
  F: Fn(&'a I) -> bool + 'a,
  I: PartialEq + 'a, {
  FuncParser::new(move |mut parse_context: ParseContext<'a, I>| {
    let input = parse_context.input();
    if let Some(actual) = input.get(0) {
      if f(actual) {
        return ParseResult::successful(parse_context.with_same_state(), actual, 1);
      }
    }
    let offset = parse_context.offset();
    let msg = format!("offset: {}", offset);
    parse_context.next_mut();
    let pe = ParseError::of_mismatch(parse_context, 1, msg);
    ParseResult::failed_with_uncommitted(pe)
  })
}

pub fn elm_ref<'a, I>(element: I) -> impl Parser<'a, I, &'a I>
where
  I: PartialEq + 'a, {
  elm_pred_ref(move |actual| *actual == element)
}

pub fn elm_any_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(|_| true)
}

pub fn elm_space_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_space)
}

pub fn elm_multi_space_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_multi_space)
}

pub fn elm_alpha_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_alpha)
}

pub fn elm_alpha_digit_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_alpha_digit)
}

pub fn elm_digit_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_digit)
}

pub fn elm_hex_digit_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_hex_digit)
}

pub fn elm_oct_digit_ref<'a, I>() -> impl Parser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_oct_digit)
}

pub fn elm_ref_of<'a, I, S>(set: &'a S) -> impl Parser<'a, I, &'a I>
where
  I: PartialEq + Display + 'a,
  S: Set<I> + ?Sized, {
  FuncParser::new(move |mut parse_context| {
    let input = parse_context.input();
    if let Some(s) = input.get(0) {
      if set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect one of: {}, found: {}", set.to_str(), s);
        parse_context.next_mut();
        let pe = ParseError::of_mismatch(parse_context, 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context))
    }
  })
}

pub fn elm_ref_in<'a, I>(start: I, end: I) -> impl Parser<'a, I, &'a I>
where
  I: PartialEq + PartialOrd + Display + 'a, {
  FuncParser::new(move |mut parse_context| {
    let set = start..=end;
    let input = parse_context.input();
    if let Some(s) = input.get(0) {
      if set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
        parse_context.next_mut();
        let pe = ParseError::of_mismatch(parse_context, 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context))
    }
  })
}

pub fn elm_ref_from_until<'a, I>(start: I, end: I) -> impl Parser<'a, I, &'a I>
where
  I: PartialEq + PartialOrd + Display + 'a, {
  FuncParser::new(move |mut parse_context| {
    let set = start..end;
    let input = parse_context.input();
    if let Some(s) = input.get(0) {
      if set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
        parse_context.next_mut();
        let pe = ParseError::of_mismatch(parse_context, 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context))
    }
  })
}

pub fn none_ref_of<'a, I, S>(set: &'a S) -> impl Parser<'a, I, &'a I>
where
  I: PartialEq + Display + 'a,
  S: Set<I> + ?Sized, {
  FuncParser::new(move |mut parse_context| {
    let input = parse_context.input();
    if let Some(s) = input.get(0) {
      if !set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect none of: {}, found: {}", set.to_str(), s);
        parse_context.next_mut();
        let pe = ParseError::of_mismatch(parse_context, 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context))
    }
  })
}
