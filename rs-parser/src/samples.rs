// use nom::{space, alphanumeric, multispace, is_alphanumeric, be_i32};
// // use nom::types::CompleteByteSlice;
// use nom::IResult;
// use std::result::Result;

// use std::{str,Utf8Error};
// // use std::collections::HashMap;

// fn take_wrapper(input: &[u8], i: u8) -> IResult<&[u8],&[u8]> {
//     take!(input, i)
// }

// // will make a parser taking 20 bytes
// named!(parser, apply!(take_wrapper, 2));

// #[test]
// fn parse_as_test() {
//   let input = b"aaaa";

//   let res = parser(input);
//   assert_eq!(
//     res,
//     IResult::Done(&b"aa"[..], &b"aa"[..])
//   );
// }



// use nom::{is_alphanumeric, be_i32};
// use nom::types::CompleteByteSlice;
use nom::IResult;

use std::str;
// use std::collections::HashMap;

fn take_char(c: u8, i: u8) -> bool {
  c == i
}

named!(a_s<&[u8], &str>,
  map_res!(
    take_while!(apply!(take_char, b'a')),
    str::from_utf8
  )
);
named!(b_s<&[u8], &str>,
  map_res!(
    take_while!(apply!(take_char, b'b')),
    str::from_utf8
  )
);
named!(a_and_b_s<&[u8], (&str, &str)>, tuple!(a_s, b_s));
named!(seq_s<&[u8], Vec<(&str, &str)> >,
  many0!(a_and_b_s)
);

pub fn parse(input: &[u8]) {
  let res = seq_s(input);
  println!("{:?}", res);
  match res {
    IResult::Done(i, o) => println!(
      "i: {:?} | o: {:?}",
      str::from_utf8(i),
      o),
    _ => println!("error: {:?}", res),
  }
}

#[test]
fn test_parse_seq_s() {
  let input = b"aabba";

  let res = seq_s(input);
  println!("{:?}", res);
  assert_eq!(
    res,
    IResult::Done(
      &b""[..],
      vec![
        ("aa", "bb"),
        ("a", "")
      ]
    )
  );
}

#[test]
fn test_parse_a_s() {
  let input = b"aaaabba";

  let res = a_s(input);
  assert_eq!(
    res,
    IResult::Done(&b"bba"[..], "aaaa")
  );
}

#[test]
fn test_parse_b_s() {
  let input = b"bba";

  let res = b_s(input);
  assert_eq!(
    res,
    IResult::Done(&b"a"[..], "bb")
  );
}

#[test]
fn test_parse_a_and_b_s() {
  let input = b"aabba";

  let res = a_and_b_s(input);
  assert_eq!(
    res,
    IResult::Done(
      &b"a"[..],
      ("aa", "bb")
    )
  );
}