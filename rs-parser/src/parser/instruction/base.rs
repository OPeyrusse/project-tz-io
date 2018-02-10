use parser::common::{RawData, be_uint, be_u8};
use parser::instruction::{ValuePointer};

named!(pub acc_pointer<&RawData, ValuePointer>,
  do_parse!(
    tag!("ACC") >>
    idx: be_u8 >>
    (ValuePointer::ACC(idx))
  )
);
named!(pub input_port<&RawData, ValuePointer>,
  do_parse!(
    port: be_uint >>
    tag!(">") >>
    (ValuePointer::PORT(port))
  )
);
named!(pub output_port<&RawData, ValuePointer>,
  do_parse!(
    tag!(">") >>
    port: be_uint >>
    (ValuePointer::PORT(port))
  )
);
named!(pub value_pointer<&RawData, ValuePointer>,
  map!(be_uint, |value| ValuePointer::VALUE(value))
);

#[cfg(test)]
mod tests {
  use super::*;
  use parser::common::tests::assert_full_result;

  #[test]
  fn test_parse_acc_pointer_explicit() {
    let res_explicit = acc_pointer(b"ACC3");
    assert_full_result(res_explicit, ValuePointer::ACC(3));
  }

  #[test]
  fn test_parse_acc_pointer_implicit() {
    let res_implicit = acc_pointer(b"ACC1");
    assert_full_result(res_implicit, ValuePointer::ACC(1));
  }

  #[test]
  fn test_parse_input_port() {
    let res = input_port(b"12>");
    assert_full_result(res, ValuePointer::PORT(12));
  }

  #[test]
  fn test_parse_output_port() {
    let res = output_port(b">43");
    assert_full_result(res, ValuePointer::PORT(43));
  }

  #[test]
  fn test_parse_value_pointer() {
    let res = value_pointer(b"37");
    assert_full_result(res, ValuePointer::VALUE(37u32));
  }
}
