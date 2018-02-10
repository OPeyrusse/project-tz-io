use nom::{space, newline as nl};

use parser::common::{RawData, be_uint, be_u8};

#[derive(Debug, PartialEq)]
pub enum ValuePointer {
  VALUE(u32),
  ACC(u8), // Limiting to 256 values
  PORT(u32)
}

#[derive(Debug, PartialEq)]
pub enum Operation {
  MOV(ValuePointer, ValuePointer)
}

named!(acc_pointer,
  do_parse!(
    tag!("ACC") >>
    idx: map!(opt!(be_u8), |value| value.unwrap_or(1u8)) >>
    (ValuePointer::ACC(idx))
  )
);

named!(mov<&RawData, Operation>,
  do_parse!(
    tag!("MOV") >> space >>
    from: be_uint >> 
    tag!(">") >>
    space >> tag!("->") >> space >>
    to: acc_pointer >>
    (Operation::MOV(ValuePointer::PORT(from), to))
  )
);

named!(pub parse_instruction<&RawData, Operation>,
  alt!(mov | mov)
);

#[cfg(test)]
mod tests {
  use super::*;
  use parser::common::tests::assert_full_result;

  fn test_parse_acc_pointer() {
    let res_implicit = acc_pointer(b"ACC");
    assert_full_result(res_implicit, ValuePointer::ACC(1));

    let res_explicit = acc_pointer(b"ACC3");
    assert_full_result(res_explicit, ValuePointer::ACC(3));
  }
}
