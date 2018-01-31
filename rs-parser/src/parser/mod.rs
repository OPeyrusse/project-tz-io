mod address;
mod syntax;
mod common;

use nom::IResult;
use std::str;

pub fn parse(input: &common::RawData) {
  let res = syntax::node_block(input);
  println!("{:?}", res);
  match res {
    IResult::Done(i, o) => println!(
      "i: {:?} | o: {:?}",
      str::from_utf8(i),
      o),
    _ => println!("error: {:?}", res),
  }
}