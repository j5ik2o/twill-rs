use crate::core::parse_context::ParseContext;
use crate::core::parse_result::ParseResult;

mod and_then_parser;
mod attempt_parser;
mod binary_operator_parser;
mod collect_parser;
mod combinators;
mod conversion_parser;
mod fn_once_parser;
mod fn_parser;
mod logging_parser;
mod offset_parser;
mod operator_parser;
mod or_parser;
mod parser_monad;
mod rc_parser;
mod repeat_parser;
mod skip_parser;
mod transform_parser;

pub use and_then_parser::*;
pub use attempt_parser::*;
pub use binary_operator_parser::*;
pub use collect_parser::*;
pub use combinators::*;
pub use conversion_parser::*;
pub use fn_once_parser::*;
pub use fn_parser::*;
pub use operator_parser::*;
pub use or_parser::*;
pub use parser_monad::*;
pub use rc_parser::*;
pub use repeat_parser::*;
pub use skip_parser::*;
pub use transform_parser::*;

pub trait Parser<'a, I: 'a, A>: Sized + 'a {
  fn run(self, parse_context: ParseContext<'a, I>) -> ParseResult<'a, I, A>;

  fn parse(self, input: &'a [I]) -> ParseResult<'a, I, A>
  where
    Self: Sized, {
    let parse_context = ParseContext::new(input, 0);
    self.run(parse_context)
  }
}

/// Basic parser trait
pub trait ClonableParser<'a, I: 'a, A>: Parser<'a, I, A> + Clone + Sized + 'a {}

// ---
