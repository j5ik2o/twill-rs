/// The struct representing the current parsing state.
#[derive(Debug, Clone)]
pub struct ParseState<'a, I> {
  input: &'a [I],
  offset: usize,
}

impl<'a, I> ParseState<'a, I> {
  /// Creates a new ParseState with the given input and offset.
  pub fn new(input: &'a [I], offset: usize) -> Self {
    Self { input, offset }
  }

  /// Creates a new ParseState from a slice with offset 0.
  pub fn from_slice(input: &'a [I]) -> Self {
    Self::new(input, 0)
  }

  /// Returns the offset of the previous character, if any.
  pub fn last_offset(&self) -> Option<usize> {
    if self.offset > 0 {
      Some(self.offset - 1)
    } else {
      None
    }
  }

  /// Returns the current offset.
  pub fn next_offset(&self) -> usize {
    self.offset
  }

  /// Returns a new ParseState with the offset increased by the given number.
  pub fn add_offset(&self, num_chars: usize) -> ParseState<'a, I> {
    Self::new(self.input, self.offset + num_chars)
  }

  /// Returns the remaining input from the current offset.
  pub fn input(&self) -> &'a [I] {
    &self.input[self.offset..]
  }

  /// Returns a slice of the input starting from the current offset with the given length.
  pub fn slice_with_len(&self, n: usize) -> &'a [I] {
    &self.input[self.offset..self.offset + n]
  }

  /// Returns a new ParseState with the offset increased by the given number.
  pub fn next(&self, n: usize) -> ParseState<'a, I> {
    Self::new(self.input, self.offset + n)
  }
  
  /// Returns whether the input is empty from the current offset.
  pub fn is_empty(&self) -> bool {
    self.offset >= self.input.len()
  }
  
  /// Returns the original input slice.
  pub fn original_input(&self) -> &'a [I] {
    self.input
  }
}
