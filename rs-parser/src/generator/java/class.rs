use generator::java::constructs::{
  Attribute,
  Signature, 
  Method};

pub type PoolIdx = u16;

#[derive(Debug)]
pub enum PoolElement {
  Utf8Value(String),
  ClassInfo(usize)
}

#[derive(Debug)]
pub struct JavaClass {
  class_pool: Vec<PoolElement>,
  // TODO collect this information
  pub class_id: PoolIdx,
  pub super_class_id: PoolIdx,
  pub interfaces: Vec<PoolIdx>,
  pub methods: Vec<Method>
}

impl JavaClass {
  pub fn new() -> JavaClass {
    JavaClass {
      class_pool: Vec::new(),
      class_id: 0,
      super_class_id: 0,
      interfaces: Vec::new(),
      methods: Vec::new()
    }
  }

  pub fn set_class(&mut self, classname: &str) {
    let class_idx = self.map_class(classname);
    self.class_id = (class_idx + 1) as u16;
  }

  pub fn set_super_class(&mut self, classname: &str) {
    let class_idx = self.map_class(classname);
    self.class_id = (class_idx + 1) as u16;
  }

  pub fn create_method(
      &mut self, 
      access: u16,
      method_name: &str,
      signature: Signature,
      attributes: Vec<Attribute>) -> PoolIdx {
    let name_idx = self.map_utf8_value(method_name);
    let descriptor = create_descriptor(&signature);
    let descriptor_idx = self.map_utf8_value(&descriptor);

    self.methods.push(Method {
      access: access,
      name_index: name_idx as PoolIdx,
      descriptor_index: descriptor_idx as PoolIdx,
      attributes: attributes
    });

    name_idx as PoolIdx
  }

  pub fn map_class(&mut self, classname: &str) -> usize {
    let value_idx = self.map_utf8_value(classname);
    let result: Option<usize> = self.class_pool.iter().enumerate()
      .find(|&e| match e.1 {
        &PoolElement::ClassInfo(ref value) => *value == value_idx,
        _ => false
      })
      .map(|e| e.0);
    match result {
      Some(existing) => existing,
      None => {
        self.class_pool.push(PoolElement::ClassInfo(value_idx));
        self.class_pool.len() - 1
      }
    }
  }

  fn map_utf8_value(&mut self, classname: &str) -> usize {
    let result: Option<usize> = self.class_pool.iter().enumerate()
      .find(|&e| match e.1 {
        &PoolElement::Utf8Value(ref value) => value.as_str() == classname,
        _ => false
      })
      .map(|e| e.0);
    match result {
      Some(existing) => existing,
      None => {
        self.class_pool.push(PoolElement::Utf8Value(String::from(classname)));
        self.class_pool.len() - 1
      }
    }
  }
}

fn create_descriptor(signature: &Signature) -> String {
  String::from("")
}
