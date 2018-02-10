use nom::{digit, space, newline};

use std::str;

pub type RawData = [u8];

fn to<T: str::FromStr>(v: &RawData) -> Result<T, i8> {
	str::from_utf8(v).or(Err(-1))
		.and_then(|i| i.parse::<T>().or(Err(-2)))

}
fn to_u8(v: &RawData) -> Result<u8, i8> {
	to(v)
}
fn to_u32(v: &RawData) -> Result<u32, i8> {
	to(v)
}

named!(pub be_uint<&RawData, u32>, map_res!(digit, to_u32));
named!(pub be_u8<&RawData, u8>, map_res!(digit, to_u8));
named!(pub ospace<&RawData, Option<&RawData> >, opt!(space));
named!(pub eol<&RawData, ()>, do_parse!(ospace >> newline >> ()));

#[cfg(test)]
pub mod tests {
	use std::cmp::PartialEq;
	use std::fmt::Debug;

	use nom::IResult;

	use super::*;

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

	#[test]
	fn test_parse_be_uint() {
		let input = b"14";
		let res = be_uint(input);
		assert_full_result(res, 14u32);
	}

	#[test]
	fn test_parse_be_u8() {
		let input = b"4";
		let res = be_u8(input);
		assert_full_result(res, 4u8);
	}

}