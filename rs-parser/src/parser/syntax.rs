use nom::{space};

use parser::common::{RawData, be_uint, ospace, eol};
use parser::address::{Node, Port, node_header, port_ref};
use parser::instructions::{parse_instruction};

#[derive(Debug, PartialEq)]
struct InputMapping<'a> {
	from: Port<'a>,
	to: u32
}
#[derive(Debug, PartialEq)]
struct OutputMapping<'a> {
	from: u32,
	to: Port<'a>
}

// Syntax lines
named!(node_line<&RawData, &RawData>, take_while!(call!(|c| c == b'=')));
named!(code_line<&RawData, &RawData>, take_while!(call!(|c| c == b'-')));

// List of inputs
named!(input_item<&RawData, InputMapping>,
	do_parse!(
		port: port_ref >>
		space >> tag!("->") >> space >>
		input: be_uint >>
		(InputMapping { from: port, to: input })
	)
);
named!(inputs<&RawData, Vec<InputMapping> >,
	separated_list_complete!(
		do_parse!(
			ospace >> tag!(",") >> space >> ()
		),
		input_item
	)
);

// List of outputs
named!(output_item<&RawData, OutputMapping>,
	do_parse!(
		input: be_uint >>
		space >> tag!("->") >> space >>
		port: port_ref >>
		(OutputMapping { from: input, to: port })
	)
);
named!(outputs<&RawData, Vec<OutputMapping> >,
	separated_list_complete!(
		do_parse!(
			ospace >> tag!(",") >> space >>
			()
		),
		output_item
	)
);

named!(pub node_block<&RawData, Node>,
	do_parse!(
		ospace >>
		node: node_header >> eol >>
		node_line >> eol >>
		inputs: inputs >> eol >>
		code_line >> eol >>
		separated_nonempty_list_complete!(
			eol,
			do_parse!(
				ospace >>
				i: parse_instruction >>
				(i)
			)
		) >> eol >>
		code_line >> eol >>
		ospace >> outputs: outputs >> eol >>
		node_line >> eol >>
		(node)
	)
);

#[cfg(test)]
mod tests {
	use super::*;
	use parser::common::tests::{assert_result, assert_full_result};

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
	fn test_parse_inputs() {
		let res_one = inputs(b"#n:7 -> 14");
		assert_full_result(
			res_one,
			vec![
				InputMapping {
					from: Port { node: Node::Node(&"n"), port: 7u32 },
					to: 14u32,
				}
			]
		);

		let res_many = inputs(b"OUT:1 -> 2, #abc:3 -> 4");
		assert_full_result(
			res_many,
			vec![
				InputMapping {
					from: Port { node: Node::Out, port: 1u32 },
					to: 2u32
				},
				InputMapping {
					from: Port { node: Node::Node(&"abc"), port: 3u32 },
					to: 4u32
				}
			]
		);
	}

	#[test]
	fn test_parse_output_item() {
		let res_in = output_item(b"1 -> OUT:3");
		assert_full_result(
			res_in,
			OutputMapping {
				from: 1u32,
				to: Port { node: Node::Out, port: 3u32 }
			}
		);

		let res_node = output_item(b"1 -> #node:32");
		assert_full_result(
			res_node,
			OutputMapping {
				from: 1u32,
				to: Port { node: Node::Node(&"node"), port: 32u32 }
			}
		);
	}

	#[test]
	fn test_parse_outputs() {
		let res_one = outputs(b"3 -> #n:7");
		assert_full_result(
			res_one,
			vec![
				OutputMapping {
					from: 3,
					to: Port { node: Node::Node(&"n"), port: 7u32}
				}
			]
		);

		let res_many = outputs(b"1 -> OUT:2, 3 -> #abc:4");
		assert_full_result(
			res_many,
			vec![
				OutputMapping {
					from: 1u32,
					to: Port { node: Node::Out, port: 2u32 }
				},
				OutputMapping {
					from: 3u32,
					to: Port { node: Node::Node(&"abc"), port: 4u32 }
				}
			]
		);
	}

	#[test]
	fn test_parse_node_block() {
		let input = b"  Node #123
==========
IN:1 -> 1
--
MOV 1>, >1
---------
1 -> OUT:1
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			Node::Node(&"123")
		);
	}

}
