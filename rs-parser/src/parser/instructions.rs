use nom::{space, newline as nl};

use parser::common::{RawData, be_uint};

pub enum ValuePointer {
  VALUE(u32),
  ACC(u8), // Limiting to 256 values
  BAK(u8),
  PORT(u32)
}
