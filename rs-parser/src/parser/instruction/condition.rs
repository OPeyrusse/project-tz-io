use nom::{alphanumeric, space};
use std::str::from_utf8;

use parser::common::RawData;
use parser::instruction::{ValuePointer, Operation};
use parser::instruction::base::{acc_pointer, input_pointer, value_pointer};

named!(label_name<&RawData, &str>,
	map_res!(alphanumeric, from_utf8)
);

named!(pub label_operation<&RawData, Operation>,
	do_parse!(
		label: label_name >> tag!(":") >>
		(Operation::LABEL(label))
	)
);

// JMP, JEZ, JNZ, JGZ, JLZ, JRO
macro_rules! jump_fn {
	($name:ident, $pattern:expr, $cnstr:path) => {
		named!(pub $name<&RawData, Operation>,
			do_parse!(
				tag!($pattern) >> space >>
				label: label_name >>
				($cnstr(label))
			)
		);
	};
}
jump_fn!(jmp_operation, "JMP", Operation::JMP);
jump_fn!(jez_operation, "JEZ", Operation::JEZ);
jump_fn!(jnz_operation, "JNZ", Operation::JNZ);
jump_fn!(jlz_operation, "JLZ", Operation::JLZ);
jump_fn!(jgz_operation, "JGZ", Operation::JGZ);

named!(pub jro_operation<&RawData, Operation>,
	do_parse!(
		tag!("JRO") >> space >>
		value: alt!(acc_pointer | input_pointer | value_pointer) >>
		(Operation::JRO(value))
	)
);

#[cfg(test)]
mod tests {
	use super::*;

	use parser::common::tests::{assert_full_result, assert_result};

	#[test]
	fn test_parse_label_operation() {
		let res = label_operation(b"aLabel1:");
		assert_full_result(res, Operation::LABEL(&"aLabel1"));
	}

	#[test]
	fn test_parse_label_operation_with_next() {
		let res = label_operation(b"lbl: NEG");
		assert_result(res, Operation::LABEL(&"lbl"), b" NEG");
	}

	#[test]
	fn test_parse_jmp_operation() {
		let res = jmp_operation(b"JMP label");
		assert_full_result(res, Operation::JMP(&"label"));
	}

	#[test]
	fn test_parse_jez_operation() {
		let res = jez_operation(b"JEZ label");
		assert_full_result(res, Operation::JEZ(&"label"));
	}

	#[test]
	fn test_parse_jnz_operation() {
		let res = jnz_operation(b"JNZ label");
		assert_full_result(res, Operation::JNZ(&"label"));
	}

	#[test]
	fn test_parse_jlz_operation() {
		let res = jlz_operation(b"JLZ label");
		assert_full_result(res, Operation::JLZ(&"label"));
	}

	#[test]
	fn test_parse_jgz_operation() {
		let res = jgz_operation(b"JGZ label");
		assert_full_result(res, Operation::JGZ(&"label"));
	}

	#[test]
	fn test_parse_jro_operation_with_value() {
		let res = jro_operation(b"JRO 1");
		assert_full_result(res, Operation::JRO(ValuePointer::VALUE(1)));
	}

	#[test]
	fn test_parse_jro_operation_with_input() {
		let res = jro_operation(b"JRO <32");
		assert_full_result(res, Operation::JRO(ValuePointer::PORT(32)));
	}

	#[test]
	fn test_parse_jro_operation_with_acc() {
		let res = jro_operation(b"JRO ACC");
		assert_full_result(res, Operation::JRO(ValuePointer::ACC));
	}
}