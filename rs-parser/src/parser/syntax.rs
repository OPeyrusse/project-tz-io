use nom::{space};

use parser::common::{RawData, be_uint, ospace, eol};
use parser::address::{Node, Port, node_header, port_ref};
use parser::instruction::{parse_instruction, Operation, ValuePointer, MemoryPointer};
use parser::instruction::condition::label_operation;

#[derive(Debug, PartialEq)]
pub struct InputMapping<'a> {
	from: Port<'a>,
	to: u32
}
#[derive(Debug, PartialEq)]
pub struct OutputMapping<'a> {
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

named!(instruction_line<&RawData, (Option<Operation>, Operation)>,
	alt!(
		// Instruction only
		do_parse!(
			op: parse_instruction >> eol >>
			(None, op)
		) |
		// Label only
		do_parse!(
			label: label_operation >> eol >>
			(None, label)			
		) |
		// Label then insctruction
		do_parse!(
			label: label_operation >> ospace >>
			op: parse_instruction >> eol >>
			(Some(label), op)			
		)
	)
);
named!(instruction_list<&RawData, Vec<Operation> >,
	fold_many1!(instruction_line, Vec::new(), |mut acc: Vec<_>, (label_opt, op)| {
		if let Some(label) = label_opt {
			acc.push(label);
		}
    acc.push(op);
    acc
	})
);
named!(pub node_block<&RawData, (Node, Vec<InputMapping>, Vec<OutputMapping>, Vec<Operation>)>,
	do_parse!(
		ospace >>
		node: node_header >> eol >>
		node_line >> eol >>
		ospace >> inputs: inputs >> eol >>
		code_line >> eol >>
		ops: instruction_list >>
		code_line >> eol >>
		ospace >> outputs: outputs >> eol >>
		node_line >> eol >>
		(node, inputs, outputs, ops)
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
	fn test_parse_instruction_line_with_label_only() {
		let res = instruction_line(b"LBL:  \n");
		assert_full_result(
			res,
			(None, Operation::LABEL(&"LBL"))
		);
	}

	#[test]
	fn test_parse_instruction_line_with_instruction_only() {
		let res = instruction_line(b"SWP  \n");
		assert_full_result(
			res,
			(None, Operation::SWP(MemoryPointer::BAK(1)))
		);
	}

	#[test]
	fn test_parse_instruction_line_with_label_then_instruction() {
		let res = instruction_line(b"LBL:SWP \n");
		assert_full_result(
			res,
			(Some(Operation::LABEL(&"LBL")), Operation::SWP(MemoryPointer::BAK(1)))
		);
	}

	#[test]
	fn test_parse_with_consecutive_labels() {
		let res = instruction_line(b"L1: L2:\n");
		assert!(res.is_err(), true);
	}

	#[test]
	fn test_parse_instruction_list() {
		let input = b"START:
MOV <1, ACC
F1:SWP
MOV ACC, >1
JEZ F1\n";
		let res = instruction_list(input);
		assert_full_result(
			res,
			vec![
				Operation::LABEL(&"START"),
				Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
				Operation::LABEL(&"F1"),
				Operation::SWP(MemoryPointer::BAK(1)),
				Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1)),
				Operation::JEZ(&"F1")
			]
		);

	}

	#[test]
	fn test_parse_node_block() {
		let input = b"  Node #123
==========
IN:1 -> 1
--
MOV <1, ACC
SWP
MOV ACC, >1
---------
1 -> OUT:1
=======
";

		let res = node_block(input);
		assert_full_result(
			res,
			(
				Node::Node(&"123"),
				vec![
					InputMapping {
						from: Port { node: Node::In, port: 1 },
						to: 1
					}
				],
				vec![
					OutputMapping {
						from: 1,
						to: Port { node: Node::Out, port: 1 }
					}
				],
				vec![
					Operation::MOV(ValuePointer::PORT(1), ValuePointer::ACC),
					Operation::SWP(MemoryPointer::BAK(1)),
					Operation::MOV(ValuePointer::ACC, ValuePointer::PORT(1))
				]
			)
		);
	}

}
