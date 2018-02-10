mod base;
mod mov;

use parser::common::RawData;
use parser::instruction::mov::mov_operation;

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

named!(pub parse_instruction<&RawData, Operation>,
  alt!(mov_operation | mov_operation)
);