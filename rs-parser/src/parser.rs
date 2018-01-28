use nom::{space, newline, is_alphanumeric, be_u32};
use nom::IResult;

use std::str;
use std::fmt;

type RawData = [u8];
#[derive(PartialEq)]
enum Node<'a> {
    In,
    Out,
    Node(&'a str)
}

impl<'a> fmt::Debug for Node<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Node::In => write!(f, "<IN>"),
      &Node::Out => write!(f, "<OUT>"),
      &Node::Node(id) => write!(f, "Node#{}", id)
    }
  }
}

named!(node_line<&RawData, &RawData>, take_while!(call!(|c| c == b'=')));
named!(code_line<&RawData, &RawData>, take_while!(call!(|c| c == b'-')));

named!(input_node<&RawData, Node>,
  do_parse!(tag!("IN") >> (Node::In))
);
named!(output_node<&RawData, Node>,
  do_parse!(tag!("OUT") >> (Node::Out))
);
named!(node_id<&RawData, Node>,
  do_parse!(
    tag!("#") >>
    id: map_res!(
      take_while!(is_alphanumeric),
      str::from_utf8
    ) >>
    (Node::Node(id))
  )
);
named!(node_ref<&RawData, Node>,
  alt!(input_node | output_node | node_id)
);
named!(node_header<&RawData, Node>,
  do_parse!(
    opt!(space) >>
    tag!("Node") >>
    opt!(space) >>
    id: node_id >>
    opt!(space) >>
    (id)
  )
);

named!(port_id<&RawData, (Node, u32)>,
  do_parse!(
    id: node_ref >>
    tag!(":") >>
    port: be_u32 >>
    (id, port)
  )
);

pub fn parse(input: &RawData) {
  let res = node_header(input);
  println!("{:?}", res);
  match res {
    IResult::Done(i, o) => println!(
      "i: {:?} | o: {:?}",
      str::from_utf8(i),
      o),
    _ => println!("error: {:?}", res),
  }
}

// fn assert_result!(res, value, remaining) {
//   assert_eq!(
//     res,
//     IResult::Done(
//       remaining[..],
//       value
//     )
//   );
// }

#[test]
fn test_parse_input_node() {
  let input = b"IN aa";
  let res = input_node(input);
  assert_eq!(
    res,
    IResult::Done(
      &b" aa"[..],
      Node::In
    )
  );
}

#[test]
fn test_parse_node_id() {
  let input = b"#abc42";
  let res = node_id(input);
  assert_eq!(
    res,
    IResult::Done(
      &b""[..],
      Node::Node(&"abc42")
    )
  );
}

#[test]
fn test_parse_node_header() {
  let input = b" Node #a1 ";

  let res = node_header(input);
  assert_eq!(
    res,
    IResult::Done(
      &b""[..],
      Node::Node(&"a1"))
  );
}

#[test]
fn test_parse_node_line() {
  let input = b"===
rest";

  let res = node_line(input);
  assert_eq!(
    res,
    IResult::Done(
      &b"\nrest"[..],
      &b"==="[..])
  );
}
