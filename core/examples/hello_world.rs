use twill_core::prelude::*;

fn main() {
  let input = b"hello world";
  let lp = elm_ref(b'\'');
  let p = (seq(b"hello") + elm_space() + seq(b"world")).collect();
  let rp = elm_ref(b'\'') + elm_ref(b';');
  // let s = surround(
  //   lp,
  //   p,
  //   rp
  // );
  // let parser = p.map_res(std::str::from_utf8);
  let result = p.parse(input).to_result().unwrap();

  println!("{:?}", result);
}
