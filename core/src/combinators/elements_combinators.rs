use crate::prelude::*;
use crate::util::Set;
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
pub fn elm_pred_ref<'a, I: 'a, F>(
  f: F,
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  F: Fn(&'a I) -> bool + 'a,
  I: PartialEq + Debug + 'a, {
  Parser::new(move |parse_context: ParseContext<'a, I>| {
    log::debug!("elm_pred_ref: start");
    let input = parse_context.input();
    if let Some(actual) = input.first() {
      if f(actual) {
        log::debug!("parse_context: {:?}", parse_context);
        log::debug!("elm_pred_ref: actual: {:?}", actual);
        log::debug!("elm_pred_ref: success");
        return ParseResult::successful(parse_context.with_same_state(), actual, 1);
      }
    }
    let offset = parse_context.next_offset();
    let msg = format!("offset: {}", offset);
    let ps = parse_context.add_offset(1);
    let pe = ParseError::of_mismatch(input, ps.next_offset(), 1, msg);
    log::debug!("elm_pred_ref: failed");
    ParseResult::failed_with_uncommitted(parse_context, pe)
  })
}

pub fn elm_pred<'a, I: 'a, F>(f: F) -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a>
where
  F: Fn(&'a I) -> bool + 'a,
  I: PartialEq + Debug + Clone + 'a, {
  elm_pred_ref(f).map(Clone::clone)
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
pub fn elm_ref<'a, I>(
  element: I,
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: PartialEq + Debug + Clone + 'a, {
  elm_pred_ref(move |actual| *actual == element.clone())
}

/// Returns a [ClonableParser] that parses the specified element.(for value)
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
/// let parser = elm('x');
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), input[0]);
/// ```
pub fn elm<'a, I>(element: I) -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a>
where
  I: PartialEq + Debug + Clone + 'a, {
  elm_ref(element).map(Clone::clone)
}

pub fn elm_any_ref<'a, I>() -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(|_| true)
}

pub fn elm_any<'a, I>() -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a>
where
    I: Element + PartialEq + Clone + 'a,
{
  elm_any_ref().map(Clone::clone)
}

pub fn elm_space_ref<'a, I>() -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_space)
}

pub fn elm_space<'a, I: Element + PartialEq + Clone + 'a>(
) -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a> {
  elm_space_ref().map(Clone::clone)
}

pub fn elm_multi_space_ref<'a, I>(
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_multi_space)
}

pub fn elm_multi_space<'a, I>(
) -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a>
where
    I: Element + PartialEq + Clone + 'a,
{
  elm_multi_space_ref().map(Clone::clone)
}

pub fn elm_alpha_ref<'a, I>() -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_alpha)
}

pub fn elm_alpha_digit_ref<'a, I>(
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_alpha_digit)
}

pub fn elm_digit_ref<'a, I>() -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: Element + PartialEq + 'a, {
  elm_pred_ref(Element::is_ascii_digit)
}

pub fn elm_digit_1_9_ref<'a, I>() -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
    I: Element + PartialEq + 'a, {
  elm_digit_ref().with_filter_not(|c: &&I| c.is_ascii_digit_zero())
}


pub fn elm_hex_digit_ref<'a, I>() -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
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
pub fn elm_oct_digit_ref<'a, I>() -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
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
/// let parser = elm_of_ref("xyz").of_many1().map(|chars| chars.into_iter().map(|c| *c).collect::<String>());
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_of_ref<'a, I, S>(
  set: &'a S,
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: PartialEq + Display + Debug + Clone + 'a,
  S: Set<I> + ?Sized, {
  // let set_ptr = set as *const S;
  Parser::new(move |parse_context| {
    // let set = unsafe { &*set_ptr };
    let input = parse_context.input();
    if let Some(s) = input.first() {
      if set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect one of: {}, found: {}", set.to_str(), s);
        let pc = parse_context.add_offset(1);
        let pe = ParseError::of_mismatch(input, pc.next_offset(), 1, msg);
        ParseResult::failed_with_uncommitted(parse_context, pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
    }
  })
}

pub fn elm_of<'a, I, S>(
  set: &'a S,
) -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a>
where
    I: PartialEq + Display + Debug + Clone + 'a,
    S: Set<I> + ?Sized,
{
  elm_of_ref(set).map(Clone::clone)
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
/// let parser = elm_in_ref('x', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_in_ref<'a, I>(
  start: I,
  end: I,
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: PartialEq + PartialOrd + Display + Debug + Copy + 'a, {
  Parser::new(move |parse_context| {
    let set = start..=end;
    let input = parse_context.input();
    if let Some(s) = input.first() {
      if set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
        let pc = parse_context.add_offset(1);
        let input = parse_context.input();
        let pe = ParseError::of_mismatch(input, pc.next_offset(), 1, msg);
        ParseResult::failed_with_uncommitted(parse_context, pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
    }
  })
}

pub fn elm_in<'a, I>(
  start: I,
  end: I,
) -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a>
where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a,
{
  elm_in_ref(start, end).map(Clone::clone)
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
/// let parser = elm_from_until_ref('w', 'z').of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn elm_from_until_ref<'a, I>(
  start: I,
  end: I,
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: PartialEq + PartialOrd + Display + Debug + Copy + 'a, {
  // クローン可能なパーサーを実装
  Parser::new(move |parse_context| {
    let set = start..end;
    let input = parse_context.input();
    if let Some(s) = input.first() {
      if set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect elm of: {}, found: {}", set.to_str(), s);
        let pc = parse_context.add_offset(1);
        let pe = ParseError::of_mismatch(input, pc.next_offset(), 1, msg);
        ParseResult::failed_with_uncommitted(parse_context, pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
    }
  })
}

pub fn elm_from_until<'a, I>(
  start: I,
  end: I,
) -> Parser<'a, I, I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, I> + 'a>
where
    I: PartialEq + PartialOrd + Display + Debug + Copy + 'a,
{
  elm_from_until_ref(start, end).map(Clone::clone)
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
/// let parser = none_of_ref("abc").of_many1().map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), text);
/// ```
pub fn none_of_ref<'a, I, S>(
  set: &'a S,
) -> Parser<'a, I, &'a I, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a I> + 'a>
where
  I: PartialEq + Display + Debug + Clone + 'a,
  S: Set<I> + ?Sized, {
  Parser::new(move |parse_context| {
    let input = parse_context.input();
    if let Some(s) = input.first() {
      if !set.contains(s) {
        ParseResult::successful(parse_context, s, 1)
      } else {
        let msg = format!("expect none of: {}, found: {}", set.to_str(), s);
        let pc = parse_context.add_offset(1);
        let pe = ParseError::of_mismatch(input, pc.next_offset(), 1, msg);
        ParseResult::failed_with_uncommitted(parse_context, pe)
      }
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
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
pub fn seq<'a, 'b, I>(
  seq: &'b [I],
) -> Parser<'a, I, &'a [I], impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, &'a [I]> + 'a>
where
  I: PartialEq + Debug + Clone + 'a,
  'b: 'a, {
  Parser::new(move |parse_context| {
    log::debug!("seq: start");
    log::debug!("seq: {:?}", seq);
    let input = parse_context.input();
    let mut index = 0;
    loop {
      if index == seq.len() {
        return ParseResult::successful(parse_context, seq, index);
      }
      if let Some(str) = input.get(index) {
        if seq[index] != *str {
          let msg = format!("seq {:?} expect: {:?}, found: {:?}", seq, seq[index], str);
          let pc = parse_context.add_offset(index);
          let input = parse_context.input();
          let pe = ParseError::of_mismatch(input, pc.next_offset(), index, msg);
          log::debug!("seq: failed: {:?}", pe);
          return ParseResult::failed(parse_context, pe, (index != 0).into());
        }
      } else {
        let pe = ParseError::of_in_complete();
        log::debug!("seq: failed: {:?}", pe);
        return ParseResult::failed_with_uncommitted(parse_context, pe);
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
pub fn tag<'a, 'b>(
  tag: &'b str,
) -> Parser<'a, char, String, impl Fn(ParseContext<'a, char>) -> ParseResult<'a, char, String> + 'a>
where
  'b: 'a, {
  Parser::new(move |parse_context| {
    let input: &[char] = parse_context.input();
    let mut index = 0;
    for c in tag.chars() {
      if let Some(&actual) = input.get(index) {
        if c != actual {
          let msg = format!("tag {:?} expect: {:?}, found: {}", tag, c, actual);
          let pc = parse_context.add_offset(index);
          let input = parse_context.input();
          let pe = ParseError::of_mismatch(input, pc.next_offset(), index, msg);
          return ParseResult::failed(parse_context, pe, (index != 0).into());
        }
      } else {
        return ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete());
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
pub fn tag_no_case<'a, 'b>(
  tag: &'b str,
) -> Parser<'a, char, String, impl Fn(ParseContext<'a, char>) -> ParseResult<'a, char, String> + 'a>
where
  'b: 'a, {
  Parser::new(move |parse_context| {
    let input = parse_context.input();
    let mut index = 0;
    for c in tag.chars() {
      if let Some(actual) = input.get(index) {
        if !c.eq_ignore_ascii_case(actual) {
          let msg = format!("tag_no_case {:?} expect: {:?}, found: {}", tag, c, actual);
          let ps = parse_context.add_offset(index);
          let input = parse_context.input();
          let offset = parse_context.next_offset();
          let pe = ParseError::of_mismatch(input, offset, index, msg);
          return ParseResult::failed(ps, pe, (index != 0).into());
        }
      } else {
        return ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete());
      }
      index += 1;
    }
    ParseResult::successful(parse_context, tag.to_string(), index)
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
pub fn regex<'a>(
  pattern: &str,
) -> Parser<'a, char, String, impl Fn(ParseContext<'a, char>) -> ParseResult<'a, char, String> + 'a> {
  let pattern = if !pattern.starts_with("^") {
    format!("^{}", pattern)
  } else {
    pattern.to_string()
  };
  let regex = Regex::new(&pattern).unwrap();
  Parser::new(move |parse_context| {
    let input: &[char] = parse_context.input();
    log::debug!("regex: input = {:?}", input);
    let str = String::from_iter(input);
    if let Some(captures) = regex.captures(&str).as_ref() {
      match captures.get(0) {
        Some(m) => ParseResult::successful(parse_context, m.as_str().to_string(), m.as_str().len()),
        _ => {
          let msg = format!("regex {:?} found: {:?}", regex, str);
          let input = parse_context.input();
          let offset = parse_context.next_offset();
          let pe = ParseError::of_mismatch(input, offset, str.len(), msg);
          ParseResult::failed(parse_context, pe, (captures.len() != 0).into())
        }
      }
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
    }
  })
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  #[test]
  fn test_elm_ref_in_success_1() {
    let text = "abc";
    let input = text.chars().collect::<Vec<_>>();
    let char_range = ('a', 'c');
    let p = elm_in_ref(char_range.0, char_range.1);

    let result = p.parse(&input[0..]);
    assert!(result.is_success());
    assert_eq!(*result.success().unwrap(), 'a');

    let result = p.parse(&input[1..]);
    assert!(result.is_success());
    assert_eq!(*result.success().unwrap(), 'b');

    let result = p.parse(&input[2..]);
    assert!(result.is_success());
    assert_eq!(*result.success().unwrap(), 'c');
  }
  #[test]
  fn test_seq_success_0() {
    let text: &str = "abc";
    let input = text.as_bytes();

    let parser = seq(b"abc").collect().map_res(std::str::from_utf8);

    let result: ParseResult<u8, &str> = parser.parse(input);

    assert!(result.is_success());
    assert_eq!(result.consumed_count(), 3);
    assert_eq!(result.success().unwrap(), text);
  }

  #[test]
  fn test_seq_success_1() {
    let input = b"abc";
    let parser = seq(b"abc");
    let result = parser.parse(input);

    assert!(result.is_success());
    assert_eq!(result.clone().success().unwrap(), b"abc");
    assert_eq!(result.consumed_count(), 3);
  }

  #[test]
  fn test_seq_success_2() {
    let input = b"abcdef";
    let parser = seq(b"abc");
    let result = parser.parse(input);

    assert!(result.is_success());
    assert_eq!(result.consumed_count(), 3);
    assert_eq!(result.success().unwrap(), b"abc");
  }

  #[test]
  fn test_seq_failure() {
    let input = b"ab";
    let parser = seq(b"abc");
    let result = parser.parse(input);

    assert!(result.is_failure());
    assert_eq!(result.consumed_count(), 0);
    assert!(result.failure().unwrap().is_in_complete());
  }
}
