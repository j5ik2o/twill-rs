use std::fmt::Debug;

/// A structure to hold parsing context information
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseContext<'a, I> {
  input: &'a [I],
  offset: usize,
}

impl<'a, I> ParseContext<'a, I> {
  /// Create a new ParseContext
  pub fn new(input: &'a [I], offset: usize) -> Self {
    Self { input, offset }
  }

  /// Get the last offset if available
  pub fn last_offset(&self) -> Option<usize> {
    if self.offset > 0 {
      Some(self.offset - 1)
    } else {
      None
    }
  }

  /// Get the current offset
  pub fn next_offset(&self) -> usize {
    self.offset
  }

  /// Create a new context by advancing n positions
  pub fn add_offset(&self, n: usize) -> ParseContext<'a, I> {
    Self::new(self.input, self.offset + n)
  }

  /// Get the remaining input slice
  pub fn input(&self) -> &'a [I]
  where
    I: Debug, {
    log::debug!("input::offset = {:?}", self.offset);
    &self.input[self.offset..]
  }

  /// Get a slice of specified length from the current position
  pub fn slice_with_len(&self, n: usize) -> &'a [I]
  where
    I: Debug, {
    log::debug!("slice_with_len::offset = {:?}", self.offset);
    log::debug!("slice_with_len::n = {:?}", n);
    &self.input[self.offset..self.offset + n]
  }

  pub fn slice_with_offset_len(&self, offset: usize, n: usize) -> &'a [I]
  where
    I: Debug, {
    &self.input[offset..offset + n]
  }

  /// Get the original input
  pub fn original_input(&self) -> &'a [I] {
    self.input
  }

  /// Get total input length
  pub fn total_length(&self) -> usize {
    self.input.len()
  }

  /// Check if we're at the end of input
  pub fn is_end(&self) -> bool {
    self.offset >= self.input.len()
  }

  /// Get the remaining input length
  pub fn remaining(&self) -> usize {
    if self.offset < self.input.len() {
      self.input.len() - self.offset
    } else {
      0
    }
  }

  /// Create a new context with the same state (same input and offset)
  pub fn with_same_state(&self) -> Self {
    Self::new(self.input, self.offset)
  }
}
