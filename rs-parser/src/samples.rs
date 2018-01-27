use nom::{space, alphanumeric, multispace, is_alphanumeric, be_i32};
// use nom::types::CompleteByteSlice;
use nom::IResult;
use std::result::Result;

use std::{str,Utf8Error};
// use std::collections::HashMap;

fn take_wrapper(input: &[u8], i: u8) -> IResult<&[u8],&[u8]> {
    take!(input, i)
}

// will make a parser taking 20 bytes
named!(parser, apply!(take_wrapper, 2));

#[test]
fn parse_as_test() {
  let input = b"aaaa";

  let res = parser(input);
  assert_eq!(
    res,
    IResult::Done(&b"aa"[..], &b"aa"[..])
  );
}