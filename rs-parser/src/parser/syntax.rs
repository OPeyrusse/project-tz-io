use nom::{space, newline};
use nom::IResult;

use parser::common::{RawData, be_uint};
use parser::address::{Node, Port, node_header, port_ref};

struct InputMapping<'a> {
	from: Port<'a>,
	to: u32
}

named!(node_line<&RawData, &RawData>, take_while!(call!(|c| c == b'=')));
named!(code_line<&RawData, &RawData>, take_while!(call!(|c| c == b'-')));
named!(input_item<&RawData, InputMapping>,
	do_parse!(
		opt!(space) >>
		port: port_ref >>
		space >> tag!("->") >> space >>
		input: be_uint >>
		(InputMapping { from: port, to: input })
	)
);
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

	#[test]
	fn test_parse_node_line() {
		let input = b"===\nrest";

		let res = node_line(input);
		assert_eq!(
			res,
			IResult::Done(
				&b"\nrest"[..],
				&b"==="[..])
		);
	}

	#[test]
	fn test_parse_code_line() {
		let input = b"----\nrest";

		let res = code_line(input);
		assert_eq!(
			res,
			IResult::Done(
				&b"\nrest"[..],
				&b"----"[..])
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
		assert_eq!(
			res,
			IResult::Done(
				&b""[..],
				Node::Node(&"123")
			)
		);
	}

}
