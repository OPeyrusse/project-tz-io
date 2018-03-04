use nom::space;

use parser::common::{RawData, ospace};
use parser::instruction::{ValuePointer, Operation};
use parser::instruction::base::*;

named!(mov_from_in<&RawData, Operation>,
  do_parse!(
    tag!("MOV") >> space >>
    from: input_pointer >>
    ospace >> tag!(",") >> ospace >>
    to: alt!(acc_pointer | nil_pointer | output_pointer) >>
    (Operation::MOV(from, to))
  )
);
named!(mov_to_out<&RawData, Operation>,
  do_parse!(
    tag!("MOV") >> space >>
    from: alt!(acc_pointer | nil_pointer | value_pointer) >>
    ospace >> tag!(",") >> ospace >>
    to: output_pointer >>
    (Operation::MOV(from, to))
  )
);
named!(mov_accs<&RawData, Operation>,
  do_parse!(
    tag!("MOV") >> space >>
    from: alt!(value_pointer | acc_pointer | nil_pointer ) >>
    ospace >> tag!(",") >> ospace >>
    to: acc_pointer >>
    (Operation::MOV(from, to))
  )
);
named!(pub mov_operation<&RawData, Operation>,
  alt!(mov_from_in | mov_to_out | mov_accs)
);

#[cfg(test)]
mod tests {
  use super::*;
  use parser::common::tests::assert_full_result;

  #[test]
  fn test_parse_mov_in_to_out() {
    let res = mov_operation(b"MOV <1, >2");
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
    let res = mov_operation(b"MOV <1, ACC");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::PORT(1),
        ValuePointer::ACC
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
    let res = mov_operation(b"MOV ACC, >4");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::ACC,
        ValuePointer::PORT(4)
      )
    );
  }

  #[test]
  fn test_parse_mov_value_to_acc() {
    let res = mov_operation(b"MOV 45, ACC");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(45),
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_val_to_acc() {
    let res = mov_operation(b"MOV 76, ACC");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::VALUE(76),
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_acc_to_acc() {
    let res = mov_operation(b"MOV ACC, ACC");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::ACC,
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_nil_to_acc() {
    let res = mov_operation(b"MOV NIL, ACC");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::NIL,
        ValuePointer::ACC
      )
    );
  }

  #[test]
  fn test_parse_mov_nil_to_out() {
    let res = mov_operation(b"MOV NIL, >12");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::NIL,
        ValuePointer::PORT(12)
      )
    );
  }

  #[test]
  fn test_parse_mov_in_to_nil() {
    let res = mov_operation(b"MOV <1, NIL");
    assert_full_result(
      res,
      Operation::MOV(
        ValuePointer::PORT(1),
        ValuePointer::NIL
      )
    );
  }
}
