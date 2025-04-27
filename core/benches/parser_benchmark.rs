use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use twill_core::core::{OperatorParser, ParseContext, Parser, ParserExt};
use twill_core::examples::string_parser::string;

// ユーティリティ関数 - ParseContextの作成
fn create_context(s: &'static str) -> ParseContext<'static, char> {
  let chars: Vec<char> = s.chars().collect();
  let slice = Box::leak(chars.into_boxed_slice());
  ParseContext::new(slice, 0)
}

fn bench_basic_parsers(c: &mut Criterion) {
  let mut group = c.benchmark_group("Basic Parser Comparison");

  // ベンチマーク設定
  group.warm_up_time(Duration::from_millis(500));
  group.measurement_time(Duration::from_secs(2));
  group.sample_size(20);

  // 様々な入力サイズでのベンチマーク
  let inputs = [
    "hello",                                                              // 短い入力
    "hello world",                                                        // 中程度の入力
    "hello world this is a longer input string to test performance with", // 長い入力
  ];

  for input in inputs.iter() {
    // シンプルなパーサー
    group.bench_with_input(BenchmarkId::new("string_parser", input.len()), input, |b, input| {
      b.iter(|| {
        let ctx = create_context(input);
        string("hello").parse(ctx)
      })
    });
  }

  group.finish();
}

fn bench_parser_combinators(c: &mut Criterion) {
  let mut group = c.benchmark_group("Parser Combinators");

  // ベンチマーク設定
  group.warm_up_time(Duration::from_millis(500));
  group.measurement_time(Duration::from_secs(2));
  group.sample_size(20);

  let input = "hello world";

  // map操作のベンチマーク
  group.bench_function("map", |b| {
    b.iter(|| {
      let ctx = create_context(input);
      string("hello").map(|s| s.len()).parse(ctx)
    })
  });

  // or操作のベンチマーク - 遅延評価
  group.bench_function("or", |b| {
    b.iter(|| {
      // 遅延評価による代替パーサー生成
      let ctx = create_context(input);
      string("hello").or(string("world")).parse(ctx)
    })
  });

  // and_then操作のベンチマーク
  group.bench_function("and_then", |b| {
    b.iter(|| {
      let ctx = create_context(input);
      string("hello").and_then(string(" world")).parse(ctx)
    })
  });

  // and_then_with操作のベンチマーク
  group.bench_function("and_then_with", |b| {
    b.iter(|| {
      let ctx = create_context(input);
      string("hello").and_then_with(|_| string(" world")).parse(ctx)
    })
  });

  group.finish();
}

fn bench_complex_parsers(c: &mut Criterion) {
  let mut group = c.benchmark_group("Complex Parser Comparison");

  // ベンチマーク設定
  group.warm_up_time(Duration::from_millis(500));
  group.measurement_time(Duration::from_secs(2));
  group.sample_size(20);

  let input = "hello world";

  // 複雑なパーサー構成（複数の演算を組み合わせた場合）
  group.bench_function("complex_nested", |b| {
    b.iter(|| {
      let ctx = create_context(input);
      string("hello")
        .map(|s| s.len())
        .map(|len| len * 2)
        .map(|n| n.to_string())
        .parse(ctx)
    })
  });

  // パーサーを毎回作り直す
  group.bench_function("recreated_parser", |b| {
    b.iter(|| {
      let ctx = create_context(input);
      string("hello").parse(ctx)
    })
  });

  group.finish();
}

criterion_group! {
    name = parser_benchmarks;
    config = Criterion::default().with_plots();
    targets = bench_basic_parsers, bench_parser_combinators, bench_complex_parsers
}

criterion_main!(parser_benchmarks);
