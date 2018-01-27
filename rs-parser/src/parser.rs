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

// #[test]
// fn test_parse_read_int() {
//   let input = b"4213s";

//   let res = be_i32(input);
//   match res {
//     IResult::Done(i, o) => println!(
//       "i: {:?} | o: {:?}",
//       str::from_utf8(i),
//       o),
//     _ => println!("{:?}", &res)
//   }
//   println!("-> {:?}", str::from_utf8(b"4213"));
//   assert_eq!(
//     res,
//     IResult::Done(&b"s"[..], 4213));
// }

// fn complete_byte_slice_to_str<'a>(s: CompleteByteSlice<'a>) -> Result<&'a str, str::Utf8Error> {
//   str::from_utf8(s.0)
// }

// named!(a_s	<CompleteByteSlice, &str>, map_res!(
//     // delimited!(
//       take_while!(call!(|c| c == b'a')),
//     // ),
//     complete_byte_slice_to_str
// ));

// named!(b_s	<CompleteByteSlice, &str>, map_res!(
//     // delimited!(
//       take_while!(call!(|c| c == b'b')),
//     // ),
//     complete_byte_slice_to_str
// ));


// named!(a_and_b_s	<CompleteByteSlice,(&str,&str)>,
//   do_parse!(
//     a: a_s >>
// 		b: b_s >>
// 		(a, b)
//   )
// );

// named!(seq_s<CompleteByteSlice, Vec<(&str, &str)> >,
//   map!(
//     many0!(terminated!(a_and_b_s)),
//     |vec: Vec<_>| vec.into_iter().collect()
//   )
// );

// #[test]
// fn parse_as_test() {
//   let input = CompleteByteSlice(
//     b"aaaa",
//   );

//   let res = a_s(input);
//   println!("{:?}", res);
//   match res {
//     Ok((i, o)) => println!("i: {:?} | o: {:?}", str::from_utf8(i.0), o),
//     _ => println!("error"),
//   }

//   // assert_eq!(res, Ok((ini_without_category, "category")));
// }

// #[test]
// fn parse_key_value_test() {
//   let ini_file = CompleteByteSlice(
//     b"parameter=value
// key = value2",
//   );

//   let ini_without_key_value = CompleteByteSlice(b"\nkey = value2");

//   let res = key_value(ini_file);
//   println!("{:?}", res);
//   match res {
//     Ok((i, (o1, o2))) => println!("i: {:?} | o: ({:?},{:?})", str::from_utf8(i.0), o1, o2),
//     _ => println!("error"),
//   }

//   assert_eq!(res, Ok((ini_without_key_value, ("parameter", "value"))));
// }


// #[test]
// fn parse_key_value_with_space_test() {
//   let ini_file = CompleteByteSlice(
//     b"parameter = value
// key = value2",
//   );

//   let ini_without_key_value = CompleteByteSlice(b"\nkey = value2");

//   let res = key_value(ini_file);
//   println!("{:?}", res);
//   match res {
//     Ok((i, (o1, o2))) => println!("i: {:?} | o: ({:?},{:?})", str::from_utf8(i.0), o1, o2),
//     _ => println!("error"),
//   }

//   assert_eq!(res, Ok((ini_without_key_value, ("parameter", "value"))));
// }

// #[test]
// fn parse_key_value_with_comment_test() {
//   let ini_file = CompleteByteSlice(
//     b"parameter=value;abc
// key = value2",
//   );

//   let ini_without_key_value = CompleteByteSlice(b"\nkey = value2");

//   let res = key_value(ini_file);
//   println!("{:?}", res);
//   match res {
//     Ok((i, (o1, o2))) => println!("i: {:?} | o: ({:?},{:?})", str::from_utf8(i.0), o1, o2),
//     _ => println!("error"),
//   }

//   assert_eq!(res, Ok((ini_without_key_value, ("parameter", "value"))));
// }

// #[test]
// fn parse_multiple_keys_and_values_test() {
//   let ini_file = CompleteByteSlice(
//     b"parameter=value;abc
// key = value2
// [category]",
//   );

//   let ini_without_key_value = CompleteByteSlice(b"[category]");

//   let res = keys_and_values(ini_file);
//   println!("{:?}", res);
//   match res {
//     Ok((i, ref o)) => println!("i: {:?} | o: {:?}", str::from_utf8(i.0), o),
//     _ => println!("error"),
//   }

//   let mut expected: HashMap<&str, &str> = HashMap::new();
//   expected.insert("parameter", "value");
//   expected.insert("key", "value2");
//   assert_eq!(res, Ok((ini_without_key_value, expected)));
// }

// #[test]
// fn parse_category_then_multiple_keys_and_values_test() {
//   //FIXME: there can be an empty line or a comment line after a category
//   let ini_file = CompleteByteSlice(
//     b"[abcd]
// parameter=value;abc
// key = value2
// [category]",
//   );

//   let ini_after_parser = CompleteByteSlice(b"[category]");

//   let res = category_and_keys(ini_file);
//   println!("{:?}", res);
//   match res {
//     Ok((i, ref o)) => println!("i: {:?} | o: {:?}", str::from_utf8(i.0), o),
//     _ => println!("error"),
//   }

//   let mut expected_h: HashMap<&str, &str> = HashMap::new();
//   expected_h.insert("parameter", "value");
//   expected_h.insert("key", "value2");
//   assert_eq!(res, Ok((ini_after_parser, ("abcd", expected_h))));
// }

// #[test]
// fn parse_multiple_categories_test() {
//   let ini_file = CompleteByteSlice(
//     b"[abcd]
// parameter=value;abc
// key = value2
// [category]
// parameter3=value3
// key4 = value4
// ",
//   );

//   let ini_after_parser = CompleteByteSlice(b"");

//   let res = categories(ini_file);
//   //println!("{:?}", res);
//   match res {
//     Ok((i, ref o)) => println!("i: {:?} | o: {:?}", str::from_utf8(i.0), o),
//     _ => println!("error"),
//   }

//   let mut expected_1: HashMap<&str, &str> = HashMap::new();
//   expected_1.insert("parameter", "value");
//   expected_1.insert("key", "value2");
//   let mut expected_2: HashMap<&str, &str> = HashMap::new();
//   expected_2.insert("parameter3", "value3");
//   expected_2.insert("key4", "value4");
//   let mut expected_h: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
//   expected_h.insert("abcd", expected_1);
//   expected_h.insert("category", expected_2);
//   assert_eq!(res, Ok((ini_after_parser, expected_h)));
// }