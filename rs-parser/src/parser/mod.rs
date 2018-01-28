mod address;
mod common;

use nom::IResult;
use std::str;

pub fn parse(input: &common::RawData) {
  let res = address::node_header(input);
  println!("{:?}", res);
  match res {
    IResult::Done(i, o) => println!(
      "i: {:?} | o: {:?}",
      str::from_utf8(i),
      o),
    _ => println!("error: {:?}", res),
  }
}