use twill_core::prelude::*;

fn main() {
  let text  = "ab";
  let input = text.chars().collect::<Vec<_>>();

  let parser  = (elm_ref('a') + elm_ref('b')).collect();
  let result = parser.parse(&input).to_result().unwrap();

  println!("{:?}", result);
}
