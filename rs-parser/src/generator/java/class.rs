use std::collections::HashMap;
use std::cmp::Eq;
use generator::java::constructs::{
  Attribute,
  Signature, 
  Method
};
use generator::java::constants::{
  ArrayType,
  Type
};

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

pub struct ClassPoolIter<'a> {
  values: Vec<(&'a PoolIdx, &'a PoolElement)>,
  idx: usize
}

impl<'a> Iterator for ClassPoolIter<'a> {
  type Item = (&'a PoolIdx, &'a PoolElement);

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx < self.values.len() {
      let i = self.idx;
      self.idx += 1;
      Some(self.values[i])
    } else {
      None
    }
  }
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

  pub fn size(&self) -> PoolIdx {
    self.next_idx
  }

  pub fn iter<'a>(&'a self) -> ClassPoolIter<'a> {
    let mut elements: Vec<(&PoolIdx, &PoolElement)> = self.pool.iter()
      .map(|(element, idx)| (idx, element))
      .collect();
    elements.sort_by(|a, b| a.0.cmp(b.0));
    ClassPoolIter { values: elements, idx: 0 }
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
    self.class_pool.map(PoolElement::Integer(value))
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

  /// Gets an iterator on all elements of the class pool
  /// 
  /// Elements are enumrated by increasing pool idx.
  pub fn pool_iter<'a>(&'a self) -> ClassPoolIter<'a> {
    self.class_pool.iter()
  }

  /// Gets the size of the class pool
  pub fn pool_size(&self) -> PoolIdx {
    self.class_pool.size()
  }
}

fn create_descriptor(signature: &Signature) -> String {
  let mut descriptor = String::from("(");
  for param in &signature.parameter_types {
    type_to_str(&mut descriptor, param);
  } 
  descriptor.push(')');
  { type_to_str(&mut descriptor, &signature.return_type); }

  descriptor
}

fn type_to_str(out: &mut String, t: &Type) {
  match t {
    &Type::Void => out.push('V'),
    &Type::Integer => out.push('I'),
    &Type::Object(ref c) => {
      out.push('L');
      out.push_str(c);
      out.push(';');
    },
    &Type::ObjectArray(ref dim, ref object_type) => {
      (0..*dim).for_each(|_| out.push('['));
      out.push('L');
      out.push_str(object_type);
      out.push(';');
    },
    &Type::PrimitiveArray(ref dim, ref prim_type) => {
      (0..*dim).for_each(|_| out.push('['));
      match prim_type {
        &ArrayType::BOOLEAN => out.push('Z'),
        &ArrayType::CHAR => out.push('C'),
        &ArrayType::FLOAT => out.push('F'),
        &ArrayType::DOUBLE => out.push('D'),
        &ArrayType::BYTE => out.push('B'),
        &ArrayType::SHORT => out.push('S'),
        &ArrayType::INT => out.push('I'),
        &ArrayType::LONG => out.push('J')
      }
    }
  }
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
