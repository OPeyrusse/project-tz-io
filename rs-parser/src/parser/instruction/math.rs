use nom::{space};

use parser::common::RawData;
use parser::instruction::{ValuePointer, Operation};
use parser::instruction::base::{value_pointer, input_pointer, acc_pointer};

named!(pub add_operation<&RawData, Operation>,
	do_parse!(
		tag!("ADD") >> space >>
		value: alt!(input_pointer | acc_pointer | value_pointer) >>
		(Operation::ADD(value))
	)
);

#[cfg(test)]
mod tests {
	use super::*;

	use parser::common::tests::assert_full_result;

	#[test]
	fn test_parse_add_operation_with_value() {
		let res = add_operation(b"ADD 1");
		assert_full_result(res, Operation::ADD(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_add_operation_with_input() {
		// FIXME Failing because no backtracking 17> is value or input
		let res = add_operation(b"ADD 17>");
		assert_full_result(res, Operation::ADD(ValuePointer::PORT(17)));
	}

	#[test]
	fn test_parse_add_operation_with_acc() {
		let res = add_operation(b"ADD ACC");
		assert_full_result(res, Operation::ADD(ValuePointer::ACC));
	}

}