use std::collections::HashMap;
use std::cmp::Eq;
use generator::java::constructs::{
  Attribute,
  Signature, 
  Method};

pub type PoolIdx = u16;

#[derive(Debug, PartialEq, Hash)]
pub enum PoolElement {
  Utf8Value(String),
  ClassInfo(PoolIdx),
  /// Info refering to a method
  /// Structure
  /// ```
  ///  1. Index to class info
  ///  2. INdex to a name & type info
  /// ```
  MethodRef(PoolIdx, PoolIdx),
  /// Info about a function
  /// Structure
  /// ```
  ///  1. Index to the method name info
  ///  2. INdex to the descriptor info
  /// ```
  NameAndType(PoolIdx, PoolIdx)
}

impl Eq for PoolElement {}

#[derive(Debug)]
struct ClassPool {
  pool: HashMap<PoolElement, PoolIdx>,
  next_idx: PoolIdx
}

impl ClassPool {
  fn new() -> ClassPool {
    ClassPool { 
      pool: HashMap::new(),
      next_idx: 1
    }
  }

  pub fn map(&mut self, element: PoolElement) -> PoolIdx {
    let idx = self.next_idx;
    match self.pool.insert(element, idx) {
      None => {
        self.next_idx += 1;
        idx
      },
      Some(ref i) => *i
    }
  }
}

#[derive(Debug)]
pub struct JavaClass {
  class_pool: ClassPool,
  // TODO collect this information
  pub class_id: PoolIdx,
  pub super_class_id: PoolIdx,
  pub interfaces: Vec<PoolIdx>,
  pub methods: Vec<Method>
}

impl JavaClass {
  pub fn new() -> JavaClass {
    JavaClass {
      class_pool: ClassPool::new(),
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

  fn map_name_and_type(
      &mut self, 
      method_name: &str,
      signature: &Signature) -> PoolIdx {
    let method_idx = self.map_utf8_value(method_name);
    let descr_idx = self.map_descriptor(signature);
    let name_and_type = PoolElement::NameAndType(method_idx, descr_idx);
    self.class_pool.map(name_and_type)
  }

  pub fn map_method(
      &mut self, 
      class_name: &str, 
      method_name: &str,
      signature: &Signature) -> PoolIdx {
    let class_idx = self.map_class(class_name);
    let nnt_idx = self.map_name_and_type(method_name, signature);
    let method_ref = PoolElement::MethodRef(class_idx, nnt_idx);
    self.class_pool.map(method_ref)
  }

  pub fn map_class(&mut self, classname: &str) -> PoolIdx {
    let value_idx = self.map_utf8_value(classname);
    let info = PoolElement::ClassInfo(value_idx);
    self.class_pool.map(info)
  }

  fn map_descriptor(&mut self, signature: &Signature) -> PoolIdx {
    let descriptor = create_descriptor(signature);
    self.map_utf8_value(&descriptor)
  }

  fn map_utf8_value(&mut self, value: &str) -> PoolIdx {
    let info = PoolElement::Utf8Value(String::from(value));
    self.class_pool.map(info)
  }
}

fn create_descriptor(signature: &Signature) -> String {
  panic!("To code")
}
