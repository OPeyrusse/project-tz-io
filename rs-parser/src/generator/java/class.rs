type PoolIdx = u16;

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
  pub interfaces: Vec<PoolIdx>
}

impl JavaClass {
  pub fn new() -> JavaClass {
    JavaClass {
      class_pool: Vec::new(),
      class_id: 0,
      super_class_id: 0,
      interfaces: Vec::new()
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

  fn map_class(&mut self, classname: &str) -> usize {
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