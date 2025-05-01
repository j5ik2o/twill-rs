use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use twill_core::core::parser::rc_parser::reusable_with_clone;
use twill_core::core::{BinaryOperatorParser, OrParser, ParseContext, Parser, ParserMonad};
use twill_core::examples::string_parser::string;

// ユーティリティ関数 - ParseContextの作成
fn create_context(s: &'static str) -> ParseContext<'static, char> {
  let chars: Vec<char> = s.chars().collect();
  let slice = Box::leak(chars.into_boxed_slice());
  ParseContext::new(slice, 0)
}

// 左結合演算子パーサーのベンチマーク
fn bench_left_associative_parser(c: &mut Criterion) {
  let mut group = c.benchmark_group("Left Associative Parser");

  // ベンチマーク設定
  group.warm_up_time(Duration::from_millis(500));
  group.measurement_time(Duration::from_secs(2));
  group.sample_size(20);

  // 様々な入力サイズでのベンチマーク
  let inputs = [
    "1+2",                  // 単純な式
    "1+2+3",                // 複数の演算子
    "1+2+3+4+5",            // より多くの演算子
    "1+2+3+4+5+6+7+8+9+10", // 長い式
  ];

  for input in inputs.iter() {
    // 左結合演算子パーサー
    group.bench_with_input(BenchmarkId::new("chain_left1", input.len()), input, |b, input| {
      b.iter(|| {
        let ctx = create_context(input);

        // 数値パーサー (Cloneを実装するためにRcParserでラップする)
        let digit = reusable_with_clone(
          string("1")
            .map(|_| 1)
            .or(string("2").map(|_| 2))
            .or(string("3").map(|_| 3))
            .or(string("4").map(|_| 4))
            .or(string("5").map(|_| 5))
            .or(string("6").map(|_| 6))
            .or(string("7").map(|_| 7))
            .or(string("8").map(|_| 8))
            .or(string("9").map(|_| 9))
            .or(string("10").map(|_| 10)),
        );

        // 演算子パーサー (Cloneを実装するためにRcParserでラップする)
        let add_op = reusable_with_clone(string("+").map(|_| |a: i32, b: i32| a + b));

        // 左結合演算子パーサー
        digit.chain_left1(add_op).run(ctx)
      })
    });
  }

  group.finish();
}

criterion_group! {
    name = binary_operator_benchmarks;
    config = Criterion::default().with_plots();
    targets = bench_left_associative_parser
}

criterion_main!(binary_operator_benchmarks);
