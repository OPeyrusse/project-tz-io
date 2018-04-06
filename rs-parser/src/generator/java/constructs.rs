use generator::java::constants::Type;

#[derive(Debug)]
pub struct Signature {
  pub return_type: Type,
  pub parameter_types: Vec<Type>
}

#[derive(Debug)]
pub struct Method {
  pub access: u16,
  pub name_index: u16,
  pub descriptor_index: u16
}
