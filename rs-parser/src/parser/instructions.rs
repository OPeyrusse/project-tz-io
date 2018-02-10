use nom::space;

use parser::common::{RawData, be_uint, be_u8, ospace};

// Value pointers parsing
#[derive(Debug, PartialEq)]
pub enum ValuePointer {
  VALUE(u32),
  ACC(u8), // Limiting to 256 values
  PORT(u32)
}

named!(acc_pointer<&RawData, ValuePointer>,
  do_parse!(
    tag!("ACC") >>
    idx: be_u8 >>
    (ValuePointer::ACC(idx))
  )
);
named!(input_port<&RawData, ValuePointer>,
  do_parse!(
    port: be_uint >>
    tag!(">") >>
    (ValuePointer::PORT(port))
  )
);
named!(output_port<&RawData, ValuePointer>,
  do_parse!(
    tag!(">") >>
    port: be_uint >>
    (ValuePointer::PORT(port))
  )
);
named!(value_pointer<&RawData, ValuePointer>,
  map!(be_uint, |value| ValuePointer::VALUE(value))
);

// Operation parsing
#[derive(Debug, PartialEq)]
pub enum Operation {
  MOV(ValuePointer, ValuePointer)
}

named!(mov_from_in<&RawData, Operation>,
  do_parse!(
    tag!("MOV") >> space >>
    from: input_port >>
    ospace >> tag!(",") >> ospace >>
    to: alt!(acc_pointer | output_port) >>
    (Operation::MOV(from, to))
  )
);
named!(mov_to_out<&RawData, Operation>,
  do_parse!(
    tag!("MOV") >> space >>
    from: alt!(acc_pointer | value_pointer) >>
    ospace >> tag!(",") >> ospace >>
    to: output_port >>
    (Operation::MOV(from, to))
  )
);
named!(mov_accs<&RawData, Operation>,
  do_parse!(
    tag!("MOV") >> space >>
    from: alt!(value_pointer | acc_pointer) >>
    ospace >> tag!(",") >> ospace >>
    to: acc_pointer >>
    (Operation::MOV(from, to))
  )
);
named!(mov_operation<&RawData, Operation>,
  alt!(mov_from_in | mov_to_out | mov_accs)
);

named!(pub parse_instruction<&RawData, Operation>,
  alt!(mov_operation | mov_operation)
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

  #[test]
  fn test_parse_mov_in_to_out() {
    let res = mov_operation(b"MOV 1>, >2");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::PORT(1),
        ValuePointer::PORT(2)
      )
    );
  }

  #[test]
  fn test_parse_mov_in_to_acc() {
    let res = mov_operation(b"MOV 1>, ACC2");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::PORT(1),
        ValuePointer::ACC(2)
      )
    );
  }

  #[test]
  fn test_parse_mov_value_to_out() {
    let res = mov_operation(b"MOV 12, >3");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(12),
        ValuePointer::PORT(3)
      )
    );
  }

  #[test]
  fn test_parse_mov_acc_to_out() {
    let res = mov_operation(b"MOV ACC3, >4");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::ACC(3),
        ValuePointer::PORT(4)
      )
    );
  }

  #[test]
  fn test_parse_mov_value_to_acc() {
    let res = mov_operation(b"MOV 45, ACC2");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(45),
        ValuePointer::ACC(2)
      )
    );
  }

  #[test]
  fn test_parse_mov_val_to_acc() {
    let res = mov_operation(b"MOV 76, ACC1");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(76),
        ValuePointer::ACC(1)
      )
    );
  }

  #[test]
  fn test_parse_mov_acc_to_acc() {
    let res = mov_operation(b"MOV ACC2, ACC4");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::ACC(2),
        ValuePointer::ACC(4)
      )
    );
  }
}
