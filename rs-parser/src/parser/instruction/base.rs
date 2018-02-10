// use nom::{IResult, Needed, ErrorKind};

use parser::common::{RawData, be_uint, be_u8};
use parser::instruction::{ValuePointer};

// pub fn acc_pointer(input:&RawData) -> IResult<&RawData, ValuePointer> {
//   if input.len() < 3 {
//     IResult::Incomplete(Needed::Size(3))
//   } else {
// 		if input[0] == b'A' && input[1] == b'C' && input[2] == b'C' {
// 			if input.len() == 3 {
// 				IResult::Done(&input[3..], ValuePointer::ACC(1))
// 			} else if input[3] >= b'1' && input[3] <= b'9' {
// 				let res = be_u8(&input[3..4]);
// 				res.map(|idx| ValuePointer::ACC(idx))
// 				// IResult::Done(&input[4..], ValuePointer::ACC(42))
// 			} else {
//     		IResult::Error(error_position!(ErrorKind::Custom(2), input))
// 			}
// 		} else {
//     	IResult::Error(error_position!(ErrorKind::Custom(1), input))
// 		}
//   }
// }
// named!(pub acc_pointer<&RawData, ValuePointer>,
//   do_parse!(
//     tag!("ACC") >>
//     idx: map!(opt!(be_u8), |idx| idx.unwrap_or(1)) >>
//     (ValuePointer::ACC(idx))
//   )
// );
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
    // let res_implicit = acc_pointer(b"ACC");
    // assert_full_result(res_implicit, ValuePointer::ACC(1));
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
