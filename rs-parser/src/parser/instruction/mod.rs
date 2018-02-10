mod base;
mod mov;
mod memory;
mod math;

use parser::common::RawData;
use parser::instruction::mov::mov_operation;
use parser::instruction::memory::{swp_operation, sav_operation};

#[derive(Debug, PartialEq)]
pub enum ValuePointer {
  VALUE(u32),
  ACC,
  PORT(u32)
}

// The idea is to have ACC is the top of the stack, for ADD, SUB, NEG, ...
// and have multiple BAK if needed
#[derive(Debug, PartialEq)]
pub enum MemoryPointer {
	BAK(u8) // Limiting to 256 values
}

#[derive(Debug, PartialEq)]
pub enum Operation {
  MOV(ValuePointer, ValuePointer),
	SAV(MemoryPointer),
	SWP(MemoryPointer),
  ADD(ValuePointer)
}

named!(pub parse_instruction<&RawData, Operation>,
  alt!(
    mov_operation |
    swp_operation |
    sav_operation
  )
);