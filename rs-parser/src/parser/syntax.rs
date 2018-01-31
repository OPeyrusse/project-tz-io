use nom::{space, newline};
use nom::IResult;

use std::str;

use parser::common::RawData;
use parser::address;
use parser::address::{node_header};

named!(node_line<&RawData, &RawData>, take_while!(call!(|c| c == b'=')));
named!(code_line<&RawData, &RawData>, take_while!(call!(|c| c == b'-')));
named!(pub node_block<&RawData, address::Node>,
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
				address::Node::Node(&"123")
			)
		);
	}

}
