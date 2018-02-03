use nom::{space, newline};

use parser::common::{RawData, be_uint};
use parser::address::{Node, Port, node_header, port_ref};

#[derive(Debug, PartialEq)]
struct InputMapping<'a> {
	from: Port<'a>,
	to: u32
}

// Syntax lines
named!(node_line<&RawData, &RawData>, take_while!(call!(|c| c == b'=')));
named!(code_line<&RawData, &RawData>, take_while!(call!(|c| c == b'-')));

// List of inputs
named!(input_item<&RawData, InputMapping>,
	do_parse!(
		opt!(space) >>
		port: port_ref >>
		space >> tag!("->") >> space >>
		input: be_uint >>
		(InputMapping { from: port, to: input })
	)
);
// named!(inputs<&RawData, Vec<&InputMapping> >,
// 	map_res!(
// 		do_parse!(
// 			first: input_item >>
// 			rest: many0!(
// 				do_parse!(
// 					space >> tag!(",") >> space >>
// 					item: input_item >>
// 					(item)
// 				)
// 			) >>
// 			(first, rest)
// 		),
// 	|(first, rest)| {
// 		rest.insert(0, first);
// 		rest
// 	}
// );

named!(pub node_block<&RawData, Node>,
	do_parse!(
		opt!(space) >>
		node: node_header >>
		opt!(space) >> newline >>
		node_line >> newline >>
		code_line >> newline >>
		code_line >> newline >>
		node_line >> newline >>
		(node)
	)
);

#[cfg(test)]
mod tests {
	use super::*;
	use parser::common::{assert_result, assert_full_result};

	#[test]
	fn test_parse_node_line() {
		let input = b"===\nrest";

		let res = node_line(input);
		assert_result(res, b"===", b"\nrest");
	}

	#[test]
	fn test_parse_code_line() {
		let input = b"----\nrest";

		let res = code_line(input);
		assert_result(res, b"----", b"\nrest");
	}

	#[test]
	fn test_parse_input_item() {
		let res_in = input_item(b"IN:1 -> 3");
		assert_full_result(
			res_in,
			InputMapping {
				from: Port { node: Node::In, port: 1u32 },
				to: 3u32
			}
		);

		let res_node = input_item(b"#node:32 -> 1");
		assert_full_result(
			res_node,
			InputMapping {
				from: Port { node: Node::Node(&"node"), port: 32u32 },
				to: 1u32
			}
		);
	}

	#[test]
	fn test_parse_node_block() {
		let input = b"  Node #123
==========
--
---------
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			Node::Node(&"123")
		);
	}

}
