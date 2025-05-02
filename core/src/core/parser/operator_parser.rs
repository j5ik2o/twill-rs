use crate::core::parser::and_then_parser::AndThenParser;
use crate::core::parser::attempt_parser::AttemptParser;
use crate::core::parser::collect_parser::CollectParser;
use crate::core::parser::logging_parser::LoggingParser;
use crate::core::parser::offset_parser::OffsetParser;
use crate::core::parser::or_parser::OrParser;
use crate::core::parser::parser_monad::ParserMonad;
use crate::core::parser::repeat_parser::RepeatParser;
use crate::core::parser::skip_parser::SkipParser;
use crate::core::parser::transform_parser::TransformParser;
use crate::core::parser::ClonableParser;
use crate::core::BinaryOperatorParser;

/// Trait providing parser operators,
/// This is a composite trait that combines all specialized parser operation traits
pub trait OperatorParser<'a, I: 'a, A>:
  ClonableParser<'a, I, A>
  + ParserMonad<'a, I, A>
  + OrParser<'a, I, A>
  + AndThenParser<'a, I, A>
  + AttemptParser<'a, I, A>
  + TransformParser<'a, I, A>
  + RepeatParser<'a, I, A>
  + SkipParser<'a, I, A>
  + TransformParser<'a, I, A>
  + CollectParser<'a, I, A>
  + BinaryOperatorParser<'a, I, A>
  + LoggingParser<'a, I, A>
  + OffsetParser<'a, I, A>
  + Sized
where
  Self: 'a, {
}

impl<'a, T, I: 'a, A> OperatorParser<'a, I, A> for T where
  T: ClonableParser<'a, I, A>
    + ParserMonad<'a, I, A>
    + OrParser<'a, I, A>
    + AndThenParser<'a, I, A>
    + AttemptParser<'a, I, A>
    + TransformParser<'a, I, A>
    + RepeatParser<'a, I, A>
    + SkipParser<'a, I, A>
    + TransformParser<'a, I, A>
    + CollectParser<'a, I, A>
    + BinaryOperatorParser<'a, I, A>
    + LoggingParser<'a, I, A>
    + OffsetParser<'a, I, A>
    + 'a
{
}
