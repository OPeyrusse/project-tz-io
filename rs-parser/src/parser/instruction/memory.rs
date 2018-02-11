use parser::common::RawData;
use parser::instruction::{MemoryPointer, Operation};

named!(pub swp_operation<&RawData, Operation>,
	value!(Operation::SWP(MemoryPointer::BAK(1)), tag!("SWP"))
);

named!(pub sav_operation<&RawData, Operation>,
	value!(Operation::SAV(MemoryPointer::BAK(1)), tag!("SAV"))
);

#[cfg(test)]
mod tests {
	use super::*;

	use parser::common::tests::assert_full_result;

	#[test]
	fn test_parse_swp_operation() {
		let res = swp_operation(b"SWP");
		assert_full_result(res, Operation::SWP(MemoryPointer::BAK(1)));
	}

	#[test]
	fn test_parse_swp_operation_to_idx() {
		// TODO code swap to other space
		// let res = swp_operation(b"SWP 3");
		// assert_full_result(res, Operation::SWP(MemoryPointer::BAK(3)));
	}

	#[test]
	fn test_parse_sav_operation() {
		let res = sav_operation(b"SAV");
		assert_full_result(res, Operation::SAV(MemoryPointer::BAK(1)));
	}

	#[test]
	fn test_parse_sav_operation_to_idx() {
		// TODO code save to other space
		// let res = sav_operation(b"SAV 2");
		// assert_full_result(res, Operation::SAV(MemoryPointer::BAK(2)));
	}
}