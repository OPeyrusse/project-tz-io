use std::str;
use std::fmt;

use nom::{space, is_alphanumeric};
use nom::IResult;

use parser::common::{RawData, be_uint};

#[derive(PartialEq)]
pub enum Node<'a> {
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

#[derive(Debug, PartialEq)]
pub struct Port<'a> {
	node: Node<'a>,
	port: u32
}

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
named!(pub node_ref<&RawData, Node>,
  alt!(input_node | output_node | node_id)
);

named!(pub port_ref<&RawData, Port>,
  do_parse!(
    id: node_ref >>
    tag!(":") >>
    port: be_uint >>
    (Port {node: id, port: port})
  )
);

named!(pub node_header<&RawData, Node>,
  do_parse!(
    tag!("Node") >>
    opt!(space) >>
    id: node_id >>
    (id)
  )
);

#[cfg(test)]
mod tests {
	use super::*;

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
	fn test_parse_output_node() {
		let input = b"OUT aa";
		let res = output_node(input);
		assert_eq!(
			res,
			IResult::Done(
				&b" aa"[..],
				Node::Out
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
		let input = b"Node #a1";

		let res = node_header(input);
		assert_eq!(
			res,
			IResult::Done(
				&b""[..],
				Node::Node(&"a1"))
		);
	}

	#[test]
	fn test_parse_node_ref() {
		let res_node = node_ref(b"#ref");
		assert_eq!(
			res_node,
			IResult::Done(
				&b""[..],
				Node::Node(&"ref"))
		);

		let res_in = node_ref(b"IN");
		assert_eq!(
			res_in,
			IResult::Done(
				&b""[..],
				Node::In)
		);

		let res_out = node_ref(b"OUT");
		assert_eq!(
			res_out,
			IResult::Done(
				&b""[..],
				Node::Out)
		);
	}

	#[test]
	fn test_parse_port_ref() {
		let res = port_ref(b"IN:13");
		println!("{:?}", res);
		assert_eq!(res.unwrap().1, Port{ node: Node::In, port: 13u32});
	}
}
