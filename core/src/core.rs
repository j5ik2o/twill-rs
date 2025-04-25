// 基本的なパーサーライブラリ
#[derive(Debug)]
pub enum PResult<I, O, E> {
    Ok(O, I),
    Err(E, bool),  // bool = committed?
}

// 基本的なパーサートレイト
pub trait Parser<I, O, E> {
    fn parse(&self, input: &I) -> PResult<I, O, E>;
}

// クロージャをパーサーとして扱う
impl<F, I, O, E> Parser<I, O, E> for F
where
    F: Fn(&I) -> PResult<I, O, E>,
{
    fn parse(&self, input: &I) -> PResult<I, O, E> {
        self(input)
    }
}

// パーサーの拡張メソッドを提供するトレイト
pub trait ParserExt<I, O, E>: Parser<I, O, E> + Sized {
    // 成功結果を変換
    fn map<F, O2>(self, f: F) -> impl Parser<I, O2, E>
    where
        F: Fn(O) -> O2,
    {
        move |input: &I| {
            match self.parse(input) {
                PResult::Ok(o, i) => PResult::Ok(f(o), i),
                PResult::Err(e, c) => PResult::Err(e, c),
            }
        }
    }
    
    // パーサーを選択的に適用
    fn or<P>(self, alt: P) -> impl Parser<I, O, E>
    where
        P: Parser<I, O, E>,
    {
        move |input: &I| {
            match self.parse(input) {
                PResult::Err(e, true) => PResult::Err(e, true),
                PResult::Err(_, false) => alt.parse(input),
                ok @ PResult::Ok(..) => ok,
            }
        }
    }
    
    // パーサー連鎖
    fn flat_map<F, P, O2>(self, f: F) -> impl Parser<I, O2, E>
    where
        F: Fn(O) -> P,
        P: Parser<I, O2, E>,
    {
        move |input: &I| {
            match self.parse(input) {
                PResult::Ok(o, i) => f(o).parse(&i),
                PResult::Err(e, c) => PResult::Err(e, c),
            }
        }
    }
}

// すべてのパーサーに拡張メソッドを提供
impl<T, I, O, E> ParserExt<I, O, E> for T where T: Parser<I, O, E> {}

// 常に成功するパーサー
pub fn pure<I: Clone, O: Clone, E>(value: O) -> impl Parser<I, O, E> {
    let value = value.clone();
    move |input: &I| PResult::Ok(value.clone(), input.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestError;

    #[test]
    fn test_pure() {
        let p = pure::<String, i32, TestError>(42);
        let input = "hello".to_string();
        
        match p.parse(&input) {
            PResult::Ok(val, rest) => {
                assert_eq!(val, 42);
                assert_eq!(rest, "hello");
            },
            _ => panic!("Pure parser should always succeed"),
        }
    }

    #[test]
    fn test_map() {
        let p = pure::<String, i32, TestError>(10);
        let mapped = p.map(|x| x * 3);
        match mapped.parse(&"input".to_string()) {
            PResult::Ok(val, _) => assert_eq!(val, 30),
            _ => panic!("Map should transform the output value"),
        }
    }
    
    #[test]
    fn test_flat_map() {
        let p = pure::<String, i32, TestError>(5);
        let result = p.flat_map(|x| pure::<String, String, TestError>(format!("result: {}", x)));
        match result.parse(&"input".to_string()) {
            PResult::Ok(val, _) => assert_eq!(val, "result: 5"),
            _ => panic!("flat_map should chain parsers"),
        }
    }
    
    #[test]
    fn test_choice() {
        let p1 = pure::<String, &'static str, TestError>("first");
        let p2 = pure::<String, &'static str, TestError>("second");
        let combined = p1.or(p2);
        match combined.parse(&"input".to_string()) {
            PResult::Ok(val, _) => assert_eq!(val, "first"),
            _ => panic!("Or should return first result on success"),
        }
    }
}
