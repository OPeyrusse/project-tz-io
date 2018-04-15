use generator::java::constructs::{
  Attribute,
  Signature, 
  Method};

pub type PoolIdx = u16;

#[derive(Debug, PartialEq)]
pub enum PoolElement {
  Utf8Value(String),
  ClassInfo(usize),
  /// Info refering to a method
  /// Structure
  /// ```
  ///  1. Index to class info
  ///  2. INdex to a name & type info
  /// ```
  MethodRef(usize, usize),
  /// Info about a function
  /// Structure
  /// ```
  ///  1. Index to the method name info
  ///  2. INdex to the descriptor info
  /// ```
  NameAndType(usize, usize)
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

  // pub fn map_self_method(
  //     &mut self, 
  //     method_name: &str,
  //     signature: &Signature) -> PoolIdx {
  //   let name_idx;
  //   {
  //     let class_entry = &self.class_pool[self.class_id as usize];
  //     if let &PoolElement::ClassInfo(idx) = class_entry {
  //       name_idx = idx;
  //     } else {
  //       panic!("Class idx not refering to class info. Got: {:?}", class_entry)
  //     }
  //   }

  //   let class_name: String;
  //   {
  //     let name_entry = &self.class_pool[name_idx];
  //     if let &PoolElement::Utf8Value(ref name) = name_entry {
  //       class_name = name.clone();
  //     } else {
  //       panic!("Class entry not refering to string. Got: {:?}", name_entry)
  //     }
  //   }
    
  //   self.map_method(&class_name, method_name, signature)
  // }

  pub fn map_method(
      &mut self, 
      class_name: &str, 
      method_name: &str,
      signature: &Signature) -> PoolIdx {
    0
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
  panic!("To code")
}
