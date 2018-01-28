use nom::{space, newline, is_alphanumeric, be_u32};
use nom::IResult;

use std::str;
use std::fmt;

use common::RawData;

named!(node_line<&RawData, &RawData>, take_while!(call!(|c| c == b'=')));
named!(code_line<&RawData, &RawData>, take_while!(call!(|c| c == b'-')));

#[cfg(test)]
mod tests {

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

	#[test]
	fn test_parse_code_line() {
		let input = b"----
	rest";

		let res = code_line(input);
		assert_eq!(
			res,
			IResult::Done(
				&b"\nrest"[..],
				&b"----"[..])
		);
	}

}
