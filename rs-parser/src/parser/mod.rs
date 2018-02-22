mod address;
mod syntax;
mod common;
mod instruction;

use nom::IResult;
use std::str;

pub fn parse(input: &common::RawData) {
  let res = syntax::node_list(input);
  println!("{:?}", res);
  match res {
    IResult::Done(i, o) => println!(
      "i: {:?} | o: {:?}",
      str::from_utf8(i),
      o),
    _ => println!("error: {:?}", res),
  }
}