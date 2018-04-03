use std::collections::HashMap;

type PoolIdx = u16;

#[derive(Debug)]
pub struct JavaClass<'a> {
  next_idx: PoolIdx,
  class_pool: HashMap<&'a str, PoolIdx>,
  // TODO collect this information
  pub class_id: PoolIdx,
  pub super_class_id: PoolIdx,
  pub interfaces: Vec<PoolIdx>
}

impl<'a> JavaClass<'a> {
  pub fn new() -> JavaClass<'a> {
    JavaClass { 
      next_idx: 0, 
      class_pool: HashMap::new(),
      class_id: 0,
      super_class_id: 0,
      interfaces: vec![]
    }
  }

  pub fn register_attribute(&mut self, attribute_name: &'a str) -> Result<PoolIdx, String> {
    self.register(attribute_name)
      .map_err(|idx| format!("Attribute already mapped to {}", idx))
  }

  fn register(&mut self, element_name: &'a str) -> Result<PoolIdx, PoolIdx> {
    let idx = self.next_idx;
    match self.class_pool.insert(element_name, idx) {
      None => {
        self.next_idx += 1;
        Ok(idx)
      },
      Some(i) => Err(i)
    }
  }
}