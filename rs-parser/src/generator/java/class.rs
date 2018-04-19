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
  /// Integer constant
  /// Structure
  /// ```
  ///  1. Integer value
  /// ```
  Integer(u32),
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

  pub fn get(&self, idx: &PoolIdx) -> Option<&PoolElement> {
    for (element, i) in self.pool.iter() {
      if *i == *idx {
        return Some(element);
      }
    }
    None
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

  pub fn next(&self) -> PoolIdx {
    self.next_idx
  }
}

#[derive(Debug)]
pub struct JavaClass {
  class_pool: ClassPool,
  // TODO collect this information
  pub class_id: PoolIdx,
  pub super_class_id: PoolIdx,
  pub interfaces: Vec<PoolIdx>,
  methods: Vec<Method>
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
    self.class_id = self.map_class(classname);
  }

  pub fn set_super_class(&mut self, classname: &str) {
    self.super_class_id = self.map_class(classname);
  }

  pub fn get_class_name(&self) -> Option<String> {
    self.class_pool.get(&self.class_id)
      .and_then(|element| match element {
        &PoolElement::ClassInfo(ref idx) => Some(idx),
        _ => None
      })
      .and_then(|idx| self.class_pool.get(idx))
      .and_then(|element| match element {
        &PoolElement::Utf8Value(ref value) => Some(value.clone()),
        _ => None
      })
  }

  pub fn create_integer(&mut self, value: u32) -> PoolIdx {
    self.class_pool.map(ClassPool::Integer(value))
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

#[cfg(test)]
mod tests {

  mod pool {
    use super::super::*;

    #[test]
    fn test_map_new_value() {
      let mut pool = ClassPool::new();
      let i = pool.map(PoolElement::ClassInfo(1));
      let e = pool.get(&i);
      assert_eq!(e.unwrap(), &PoolElement::ClassInfo(1));
    }

    #[test]
    fn test_map_multiple_values() {
      let mut pool = ClassPool::new();
      let i1 = pool.map(PoolElement::ClassInfo(1));
      let i2 = pool.map(PoolElement::ClassInfo(2));
      assert_ne!(i1, i2);
    }

    #[test]
    fn test_map_existing_value() {
      let mut pool = ClassPool::new();
      let i1 = pool.map(PoolElement::ClassInfo(1));
      let i2 = pool.map(PoolElement::ClassInfo(1));
      assert_eq!(i1, i2);
    }

    #[test]
    fn test_map_multiple_types() {
      let mut pool = ClassPool::new();
      let i1 = pool.map(PoolElement::ClassInfo(1));
      let i2 = pool.map(PoolElement::NameAndType(2, 3));
      assert_ne!(i1, i2);

      let e1 = pool.get(&i1).unwrap();
      assert_eq!(e1, &PoolElement::ClassInfo(1));
      let e2 = pool.get(&i2).unwrap();
      assert_eq!(e2, &PoolElement::NameAndType(2, 3));
    }

  }

  mod base {
    use super::super::*;

    #[test]
    fn test_set_class_name() {
      let mut c = JavaClass::new();
      c.set_class("a/b/C");
      assert_eq!(c.class_id, 2); // Mapping name then class info
    }

    #[test]
    fn test_get_class_name() {
      let mut c = JavaClass::new();
      c.set_class("a/b/C");
      assert_eq!(c.get_class_name().unwrap(), String::from("a/b/C"));
    }

    #[test]
    fn test_set_super_class_name() {
      let mut c = JavaClass::new();
      c.set_super_class("a/b/SC");
      assert_eq!(c.super_class_id, 2); // Mapping name then class info
    }
  }

}
