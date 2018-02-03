use nom::{digit};
use nom::IResult;

use std::str;
use std::cmp::PartialEq;
use std::fmt::Debug;

pub type RawData = [u8];

fn to_int(v: &RawData) -> Result<u32, i8> {
	str::from_utf8(v).or(Err(-1))
		.and_then(|i| i.parse::<u32>().or(Err(-2)))
}

named!(pub be_uint<&RawData, u32>, map_res!(digit, to_int));

pub fn assert_result<Result: PartialEq + Debug> (
		res: IResult<&[u8], Result>,
		value: Result,
		remaining: &RawData) {
	assert_eq!(
		res,
		IResult::Done(
			remaining,
			value
		)
	);
}

pub fn assert_full_result<Result: PartialEq + Debug> (
		res: IResult<&[u8], Result>,
		value: Result) {
	assert_result(res, value, b"");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_be_uint() {
		let input = b"14";
		let res = be_uint(input);
		assert_result(res, 14u32, b"");
	}

}