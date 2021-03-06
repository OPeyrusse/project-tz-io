use parser::common::{RawData, be_uint};
use parser::instruction::{ValuePointer, MemoryPointer};

named!(pub acc_pointer<&RawData, ValuePointer>,
	value!(ValuePointer::ACC, tag!("ACC"))
);

named!(pub nil_pointer<&RawData, ValuePointer>,
	value!(ValuePointer::NIL, tag!("NIL"))
);

named!(pub input_pointer<&RawData, ValuePointer>,
  do_parse!(
    tag!("<") >>
    port: be_uint >>
    (ValuePointer::PORT(port))
  )
);

named!(pub output_pointer<&RawData, ValuePointer>,
  do_parse!(
    tag!(">") >>
    port: be_uint >>
    (ValuePointer::PORT(port))
  )
);

named!(pub value_pointer<&RawData, ValuePointer>,
  map!(be_uint, |value| ValuePointer::VALUE(value))
);

named!(pub bak_pointer<&RawData, MemoryPointer>,
	value!(MemoryPointer::BAK(1), tag!("BAK"))
);

#[cfg(test)]
mod tests {
  use super::*;
  use parser::common::tests::assert_full_result;

  #[test]
  fn test_parse_acc_pointer() {
    let res_explicit = acc_pointer(b"ACC");
    assert_full_result(res_explicit, ValuePointer::ACC);
  }

  #[test]
  fn test_parse_nil_pointer() {
    let res_explicit = nil_pointer(b"NIL");
    assert_full_result(res_explicit, ValuePointer::NIL);
  }

  #[test]
  fn test_parse_input_pointer() {
    let res = input_pointer(b"<12");
    assert_full_result(res, ValuePointer::PORT(12));
  }

  #[test]
  fn test_parse_output_pointer() {
    let res = output_pointer(b">43");
    assert_full_result(res, ValuePointer::PORT(43));
  }

  #[test]
  fn test_parse_value_pointer() {
    let res = value_pointer(b"37");
    assert_full_result(res, ValuePointer::VALUE(37u32));
  }

	#[test]
	fn test_parse_bak_pointer() {
		let res = bak_pointer(b"BAK");
		assert_full_result(res, MemoryPointer::BAK(1));
	}
}
