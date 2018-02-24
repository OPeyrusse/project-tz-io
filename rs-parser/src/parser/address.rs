use std::str;
use std::fmt;

use nom::{space, is_alphanumeric};

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
	pub node: Node<'a>,
	pub port: u32
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
	use parser::common::tests::{assert_result, assert_full_result};

	#[test]
	fn test_parse_input_node() {
		let input = b"IN aa";
		let res = input_node(input);
		assert_result(res, Node::In, b" aa");
	}

	#[test]
	fn test_parse_output_node() {
		let input = b"OUT aa";
		let res = output_node(input);
		assert_result(res, Node::Out, b" aa");
	}

	#[test]
	fn test_parse_node_id() {
		let input = b"#abc42";
		let res = node_id(input);
		assert_full_result(res, Node::Node(&"abc42"));
	}

	#[test]
	fn test_parse_node_header() {
		let input = b"Node #a1";

		let res = node_header(input);
		assert_full_result(res, Node::Node(&"a1"));
	}

	#[test]
	fn test_parse_node_ref() {
		let res_node = node_ref(b"#ref");
		assert_full_result(res_node, Node::Node(&"ref"));

		let res_in = node_ref(b"IN");
		assert_full_result(res_in, Node::In);

		let res_out = node_ref(b"OUT");
		assert_full_result(res_out, Node::Out);
	}

	#[test]
	fn test_parse_port_ref() {
		let res = port_ref(b"IN:13");
		assert_full_result(res, Port { node: Node::In, port: 13u32 });
	}
}
