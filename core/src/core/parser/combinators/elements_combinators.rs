use crate::core::element::Element;
use crate::core::parser::FnParser;
use crate::core::util::Set;
use crate::core::{ClonableParser, ParseContext, ParseError, ParseResult};
use regex::Regex;
use std::fmt::{Debug, Display};

/// Returns a [ClonableParser] that parses the elements that satisfy the specified closure conditions.(for reference)
///
/// - f: Closure
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_pred_ref(|c| *c == 'x');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), &input[0]);
/// ```
pub fn elm_pred_ref<'a, I: 'a, F>(f: F) -> impl ClonableParser<'a, I, &'a I>
where
  F: Fn(&'a I) -> bool + Clone + 'a,
  I: PartialEq + 'a, {
  FnParser::new(move |mut parse_context: ParseContext<'a, I>| {
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

/// Returns a [ClonableParser] that parses the specified element.(for reference)
///
/// - element: element
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
///
/// let text: &str = "x";
/// let input: Vec<char> = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_ref('x');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(*result.success().unwrap(), input[0]);
/// ```
pub fn elm_ref<'a, I>(element: I) -> impl ClonableParser<'a, I, &'a I>
where
  I: PartialEq + Clone + 'a, {
  elm_pred_ref(move |actual| *actual == element.clone())
}

pub fn elm_any_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(|_| true)
}

pub fn elm_space_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_space)
}

pub fn elm_multi_space_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_multi_space)
}

pub fn elm_alpha_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_alpha)
}

pub fn elm_alpha_digit_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_alpha_digit)
}

pub fn elm_digit_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_digit)
}

pub fn elm_hex_digit_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_hex_digit)
}

/// Returns a [ClonableParser] that parses oct digits ('0'..='8').(for reference)<br/>
/// 8進の数字('0'..='8')を解析する[ClonableParser]を返します。(参照版)
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "012345678";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_oct_digit_ref().of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_oct_digit_ref<'a, I>() -> impl ClonableParser<'a, I, &'a I>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_oct_digit)
}

/// Returns a [ClonableParser] that parses the elements in the specified set. (for reference)
///
/// - set: element of sets
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_ref_of("xyz").of_many1().map(|chars| chars.into_iter().map(|c| *c).collect::<String>());
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_ref_of<'a, I, S>(set: &'a S) -> impl ClonableParser<'a, I, &'a I>
where
  I: PartialEq + Display + Clone + 'a,
  S: Set<I> + ?Sized, {
  let set_ptr = set as *const S;
  FnParser::new(move |mut parse_context| {
    let set = unsafe { &*set_ptr };
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

/// Returns a [ClonableParser] that parses the elements in the specified range. (for reference)
///
/// - start: start element
/// - end: end element
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_ref_in('x', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_ref_in<'a, I>(start: I, end: I) -> impl ClonableParser<'a, I, &'a I>
where
  I: PartialEq + PartialOrd + Display + Clone + 'a, {
  FnParser::new(move |mut parse_context| {
    let input = parse_context.input();
    if let Some(s) = input.get(0) {
      if *s >= start && *s <= end {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect elm of: {}..={}, found: {}", start, end, s);
        parse_context.next_mut();
        let pe = ParseError::of_mismatch(parse_context, 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context))
    }
  })
}

/// Returns a [ClonableParser] that parses the elements in the specified range. (for reference)
///
/// - start: a start element
/// - end: an end element, process up to the element at end - 1
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "wxy";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = elm_ref_from_until('w', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_ref_from_until<'a, I>(start: I, end: I) -> impl ClonableParser<'a, I, &'a I>
where
  I: PartialEq + PartialOrd + Display + Clone + 'a, {
  // クローン可能なパーサーを実装
  FnParser::new(move |mut parse_context| {
    let input = parse_context.input();
    if let Some(s) = input.get(0) {
      if *s >= start && *s < end {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect elm from {} until {}, found: {}", start, end, s);
        parse_context.next_mut();
        let pe = ParseError::of_mismatch(parse_context, 1, msg);
        ParseResult::failed_with_uncommitted(pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context))
    }
  })
}

/// Returns a [ClonableParser] that parses elements that do not contain elements of the specified set.(for reference)
///
/// - set: a element of sets
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "xyz";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = none_ref_of("abc").of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn none_ref_of<'a, I, S>(set: &'a S) -> impl ClonableParser<'a, I, &'a I>
where
  I: PartialEq + Display + Clone + 'a,
  S: Set<I> + ?Sized, {
  let set_ptr = set as *const S;
  FnParser::new(move |mut parse_context| {
    let set = unsafe { &*set_ptr };
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

/// Returns a [ClonableParser] that parses a sequence of elements.<br/>
/// 要素の列を解析する[ClonableParser]を返す。
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abc";
/// let input = text.as_bytes();
///
/// let parser = seq(b"abc").collect().map_res(std::str::from_utf8);
///
/// let result = parser.parse(input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn seq<'a, 'b, I>(seq: &'b [I]) -> impl ClonableParser<'a, I, Vec<I>>
where
  I: PartialEq + Debug + Clone + 'a,
  'b: 'a, {
  FnParser::new(move |mut parse_state| {
    let input = parse_state.input();
    let mut index = 0;
    loop {
      if index == seq.len() {
        return ParseResult::successful(parse_state, seq.to_vec(), index);
      }
      if let Some(str) = input.get(index) {
        if seq[index] != *str {
          let msg = format!("seq {:?} expect: {:?}, found: {:?}", seq, seq[index], str);
          parse_state.advance_mut(index);
          let pe = ParseError::of_mismatch(parse_state, index, msg);
          return ParseResult::failed(pe, (index != 0).into());
        }
      } else {
        return ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_state));
      }
      index += 1;
    }
  })
}

/// Returns a [ClonableParser] that parses a string.
///
/// - tag: a string
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = tag("abc");
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn tag<'a, 'b>(tag: &'b str) -> impl ClonableParser<'a, char, String>
where
  'b: 'a, {
  FnParser::new(move |mut parse_context| {
    let input: &[char] = parse_context.input();
    let mut index = 0;
    for c in tag.chars() {
      if let Some(&actual) = input.get(index) {
        if c != actual {
          let msg = format!("tag {:?} expect: {:?}, found: {}", tag, c, actual);
          parse_context.advance_mut(index);
          let pe = ParseError::of_mismatch(parse_context, index, msg);
          return ParseResult::failed(pe, (index != 0).into());
        }
      } else {
        return ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context));
      }
      index += 1;
    }
    ParseResult::successful(parse_context, tag.to_string(), index)
  })
}

/// Returns a [ClonableParser] that parses a string. However, it is not case-sensitive.
///
/// - tag: a string
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
///
/// let text: &str = "aBcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = tag_no_case("abc");
///
/// let result: ParseResult<char, String> = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn tag_no_case<'a, 'b>(tag: &'b str) -> impl ClonableParser<'a, char, String>
where
  'b: 'a, {
  FnParser::new(move |parse_state| {
    let input = parse_state.input();
    let mut index = 0;
    for c in tag.chars() {
      if let Some(actual) = input.get(index) {
        if !c.eq_ignore_ascii_case(actual) {
          let msg = format!("tag_no_case {:?} expect: {:?}, found: {}", tag, c, actual);
          let ps = parse_state.advance(index);
          let pe = ParseError::of_mismatch(ps, index, msg);
          return ParseResult::failed(pe, (index != 0).into());
        }
      } else {
        return ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_state));
      }
      index += 1;
    }
    ParseResult::successful(parse_state, tag.to_string(), index)
  })
}
/// Returns a [ClonableParser] that parses a string that match a regular expression.
///
/// - pattern: a regular expression
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = regex("[abc]+");
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn regex<'a>(pattern: &str) -> impl ClonableParser<'a, char, String> {
  let pattern = if !pattern.starts_with("^") {
    format!("^{}", pattern)
  } else {
    pattern.to_string()
  };
  let regex = Regex::new(&pattern).unwrap();
  FnParser::new(move |parse_context| {
    let input: &[char] = parse_context.input();
    log::debug!("regex: input = {:?}", input);
    let str = String::from_iter(input);
    if let Some(captures) = regex.captures(&str).as_ref() {
      if let Some(m) = captures.get(0) {
        let str = m.as_str();
        ParseResult::successful(parse_context, str.to_string(), str.len())
      } else {
        let msg = format!("regex {:?} found: {:?}", regex, str);
        let pe = ParseError::of_mismatch(parse_context, str.len(), msg);
        return ParseResult::failed(pe, (captures.len() != 0).into());
      }
    } else {
      return ParseResult::failed_with_uncommitted(ParseError::of_in_complete(parse_context));
    }
  })
}

#[cfg(test)]
mod tests {
  use crate::core::parser::combinators::elm_ref_in;
  use crate::core::parser::rc_parser::reusable_parser;
  use crate::core::Parser;

  #[test]
  fn test_elm_ref_in() {
    let text = "abc";
    let input = text.chars().collect::<Vec<_>>();
    // ファクトリー関数を使用してパーサーを生成
    let char_range = ('a', 'c');
    let p = reusable_parser(move || elm_ref_in(char_range.0, char_range.1));

    let result = p.clone().parse(&input[0..]);
    assert!(result.is_success());
    println!("{:?}", result.success());

    let result = p.clone().parse(&input[1..]);
    assert!(result.is_success());
    println!("{:?}", result.success());
  }
}
