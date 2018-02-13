mod base;
mod mov;
mod memory;
mod math;
pub mod condition;

use parser::common::RawData;
use parser::instruction::mov::*;
use parser::instruction::memory::*;
use parser::instruction::math::*;
use parser::instruction::condition::*;

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
pub enum Operation<'a> {
  MOV(ValuePointer, ValuePointer),
	SAV(MemoryPointer),
	SWP(MemoryPointer),
  ADD(ValuePointer),
  SUB(ValuePointer),
  NEG,
  LABEL(&'a str),
  JMP(&'a str),
  JEZ(&'a str),
  JNZ(&'a str),
  JLZ(&'a str),
  JGZ(&'a str),
  JRO(ValuePointer)
}

named!(pub parse_instruction<&RawData, Operation>,
  alt!(
    mov_operation |
    swp_operation |
    sav_operation |
    add_operation |
    sub_operation |
    neg_operation |
    // label_operation |
    jmp_operation |
    jez_operation |
    jnz_operation |
    jlz_operation |
    jgz_operation |
    jro_operation
  )
);