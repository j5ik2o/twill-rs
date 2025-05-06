use std::fmt::Debug;
use crate::prelude::*;

/// Returns a [ClonableParser] that returns an element of the specified length.
///
/// - n: Length of the reading element
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take(3).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take<'a, I: 'a>(n: usize) -> impl Parser<'a, I, &'a [I]> where I: Debug{
  RcParser::new(move |parse_context| {
    let input = parse_context.input();
    if input.len() >= n {
      let value = parse_context.slice_with_len(n);
      ParseResult::successful(parse_context, value, n)
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
    }
  })
}

/// Returns a [ClonableParser] that returns elements, while the result of the closure is true.
///
/// The length of the analysis result is not required.
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_while0(|e| match *e {
///  'a'..='c' => true,
///   _ => false
/// }).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_while0<'a, I, F>(f: F) -> impl Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + Clone + 'a,
  I: Element + 'a, {
  RcParser::new(move |parse_context| {
    let input = parse_context.input();
    let mut start: Option<usize> = None;
    let mut len = 0;
    let mut index = 0;
    while let Some(c) = input.get(index) {
      if f(c) {
        if start.is_none() {
          start = Some(index);
        }
        len += 1;
      }
      index += 1;
    }
    match start {
      Some(s) => ParseResult::successful(parse_context, &input[s..s + len], len),
      None => {
        let value = parse_context.slice_with_len(0);
        ParseResult::successful(parse_context, value, 0)
      }
    }
  })
}
/// Returns a [ClonableParser] that returns elements, while the result of the closure is true.
///
/// The length of the analysis result must be at least one element.
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_while1(|e| match *e {
///  'a'..='c' => true,
///   _ => false
/// }).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_while1<'a, I, F>(f: F) -> impl Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + Clone + 'a,
  I: Element + 'a, {
  RcParser::new(move |parse_context| {
    let input = parse_context.input();
    let mut start: Option<usize> = None;
    let mut len = 0;
    let mut index = 0;
    while let Some(c) = input.get(index) {
      if f(c) {
        if start.is_none() {
          start = Some(index);
        }
        len += 1;
      }
      index += 1;
    }
    match start {
      Some(s) => ParseResult::successful(parse_context, &input[s..s + len], len),
      None => ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete()),
    }
  })
}

/// Returns a [ClonableParser] that returns elements, while the result of the closure is true.
///
/// The length of the analysis result should be between n and m elements.
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_while_n_m(1, 3, |e| match *e {
///  'a'..='c' => true,
///   _ => false
/// }).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_while_n_m<'a, I, F>(n: usize, m: usize, f: F) -> impl Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + Clone + 'a,
  I: Element + 'a, {
  RcParser::new(move |parse_context| {
    let input = parse_context.input();
    let mut start: Option<usize> = None;
    let mut len = 0;
    let mut index = 0;
    while let Some(c) = input.get(index) {
      if f(c) {
        if start.is_none() {
          start = Some(index);
        }
        len += 1;
      }
      index += 1;
    }
    match start {
      Some(s) => {
        let str = &input[s..s + len];
        if n <= str.len() && str.len() <= m {
          ParseResult::successful(parse_context, str, len)
        } else {
          ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
        }
      }
      None => ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete()),
    }
  })
}

/// Returns a [ClonableParser] that returns a sequence up to either the end element or the element that matches the condition.
///
/// The length of the analysis result must be at least one element.
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_till0(|e| matches!(*e, 'c')).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_till0<'a, I, F>(f: F) -> impl Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + Clone + 'a,
  I: Element + 'a, {
  RcParser::new(move |parse_context| {
    let input = parse_context.input();
    let mut index = 0;
    let mut b = false;
    while let Some(c) = input.get(index) {
      if f(c) {
        b = true;
        break;
      }
      index += 1;
    }
    if b {
      let value = parse_context.slice_with_len(index + 1);
      ParseResult::successful(parse_context, value, index + 1)
    } else {
      let input = parse_context.input();
      ParseResult::successful(parse_context, input, input.len())
    }
  })
}

/// Returns a [ClonableParser] that returns a sequence up to either the end element or the element that matches the condition.
///
/// The length of the analysis result must be at least one element.
///
/// # Example
///
/// ```rust
/// # use twill_core::prelude::*;
/// # use std::iter::FromIterator;
///
/// let text: &str = "abcdef";
/// let input = text.chars().collect::<Vec<_>>();
///
/// let parser = take_till1(|e| matches!(*e, 'c')).map(String::from_iter);
///
/// let result = parser.parse(&input);
///
/// assert!(result.is_success());
/// assert_eq!(result.success().unwrap(), "abc");
/// ```
pub fn take_till1<'a, I, F>(f: F) -> impl Parser<'a, I, &'a [I]>
where
  F: Fn(&I) -> bool + Clone + 'a,
  I: Element + 'a, {
  RcParser::new(move |parse_context| {
    let input = parse_context.input();
    let mut index = 0;
    let mut b = false;
    while let Some(c) = input.get(index) {
      if f(c) {
        b = true;
        break;
      }
      index += 1;
    }
    if b {
      let value = parse_context.slice_with_len(index + 1);
      ParseResult::successful(parse_context, value, index + 1)
    } else {
      ParseResult::failed_with_uncommitted(parse_context, ParseError::of_in_complete())
    }
  })
}
