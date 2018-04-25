use generator::java::constants::{Type, ArrayType};

#[derive(Debug, PartialEq, Clone)]
pub struct Signature {
  pub return_type: Type,
  pub parameter_types: Vec<Type>
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Operation {
  /// Load a value from an array
  // aaload,
  /// Push the value into an array at a given index
  // aastore,
  /// Lods a reference of a local variable into the stack
  /// Structure
  /// ```
  ///  1. Index of the local variable
  /// ```
  aload(u8),
  /// Stores a reference into a local variable
  /// Structure
  /// ```
  ///  1. Index of the local variable
  /// ```
  astore(u8),
  /// Store an integer into an array
  iastore,
  iconst_1,
  invokespecial(u16),
  invokevirtual(u16),
  ldc(u16),
  new(u16),
  newarray(ArrayType)
}

#[derive(Debug)]
pub enum Attribute {
  /// Code attribute
  /// Structure
  /// ```
  ///  1. max stack size
  ///  2. Operations
  /// ```
  Code(u16, Vec<Operation>)
}

#[derive(Debug)]
pub struct Method {
  pub access: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes: Vec<Attribute>
}
