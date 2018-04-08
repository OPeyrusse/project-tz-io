use generator::java::constants::Type;

#[derive(Debug)]
pub struct Signature {
  pub return_type: Type,
  pub parameter_types: Vec<Type>
}

#[derive(Debug)]
pub enum Operation {
  aaload,
  aload,
  aload_0,
  iconst_1,
  iload_1,
  invokespecial(u16),
  invokevirtual(u16)
}

#[derive(Debug)]
pub enum Attribute {
  Code(u16, Vec<Operation>)
}

#[derive(Debug)]
pub struct Method {
  pub access: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub attributes: Vec<Attribute>
}
