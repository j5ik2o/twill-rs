use std::env;
use twill_core::prelude::*;

fn main() {
  env::set_var("RUST_LOG", "debug");
  let _ = env_logger::builder().is_test(true).try_init();

  let input = b"hello world";

  // パーサーを個別のステップに分解
  let hello = seq(b"hello");
  let space = elm_space();
  let world = seq(b"world");

  // 段階的に組み合わせる
  let hello_space = hello + space + world;
  let p = hello_space.collect();

  // 結果を処理
  let result = p.parse(input).to_result().unwrap();

  println!("{:?}", result);
}
