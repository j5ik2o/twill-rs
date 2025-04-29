use crate::core::util::RangeArgument;
use crate::core::{BinaryOperatorParser, Parser};
use std::fmt::Debug;

pub trait RepeatParser<'a, I: 'a, A>: Parser<'a, I, A> + BinaryOperatorParser<'a, I, A> + Sized
where
  Self: 'a, {
  // fn repeat_seq<P2, B, R>(self, range: R, separator: Option<P2>) -> impl Parser<'a, I, Vec<A>>
  // where R: RangeArgument<usize> + Debug + 'a {
  //
  // }
}
