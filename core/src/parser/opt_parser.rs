use crate::parse_context::ParseContext;
use crate::parse_result::ParseResult;
use crate::parser::{Parser, ParserRunner};

pub trait OptParser<'a, I: 'a, A>: ParserRunner<'a, I, A>
where
    Self: 'a, {
    fn opt<P>(self) -> Parser<'a, I, A, impl Fn(ParseContext<'a, I>) -> ParseResult<'a, I, A> + 'a>
    where
        A: 'a, { 
        // 概念的な仕様 = or(map(attempt(parser), Some), successful(None)) 
        // 高階関数で合成するとCloneが必須になるだろうから、クロージャーを使用せずにParser::newを使って直接実行したほうがよさそう
    }
}

/// Add Or methods to all parsers
impl<'a, T, I: 'a, A> OptParser<'a, I, A> for T where T: ParserRunner<'a, I, A> + 'a {}