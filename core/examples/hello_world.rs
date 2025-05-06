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
  let hello_space = hello + space;
  let hello_space_world = hello_space + world;
  let p = hello_space_world.collect();
  
  // 結果を処理
  let result = p.parse(input).to_result().unwrap();

  println!("{:?}", result);
}
