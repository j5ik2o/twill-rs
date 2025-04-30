use crate::core::parser::and_then_parser::AndThenParser;
use crate::core::parser::attempt_parser::AttemptParser;
use crate::core::parser::choice_parser::OrParser;
use crate::core::parser::transform_parser::TransformParser;
use crate::core::parser::Parser;
use crate::core::parser_monad::ParserMonad;

/// Trait providing parser operators
/// This is a composite trait that combines all specialized parser operation traits
pub trait OperatorParser<'a, I: 'a, A>:
  Parser<'a, I, A>
  + ParserMonad<'a, I, A>
  + OrParser<'a, I, A>
  + AttemptParser<'a, I, A>
  + AndThenParser<'a, I, A>
  + TransformParser<'a, I, A>
  + Sized
where
  Self: 'a, {
}

impl<'a, T, I: 'a, A> OperatorParser<'a, I, A> for T where
  T: Parser<'a, I, A>
    + ParserMonad<'a, I, A>
    + OrParser<'a, I, A>
    + AndThenParser<'a, I, A>
    + TransformParser<'a, I, A>
    + 'a
{
}
