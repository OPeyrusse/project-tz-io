mod class;
mod writer;
mod constants;
mod constructs;

use std::path::Path;

use parser::ParsingTree;
use parser::syntax::{NodeBlock};

// pub fn create_node_file(node_block: &NodeBlock, output_file: &Path) -> Result<(), String> {
//   let mut class = class::JavaClass::new();

//   writer::write(&class, output_file)
//     .map_err(|e| format!("Failed to write into file. Caused by {}", e))
// }

pub fn create_main_file(tree: &ParsingTree, output_file: &Path) -> Result<(), String> {
  let mut class = class::JavaClass::new();

  let mut classname = String::from("com/kineolyan/tzio/");
  classname.push_str(output_file.file_stem().unwrap().to_str().unwrap());
  classname.push_str("/Main");
  class.set_class(&classname);

  class.set_super_class(&"com/kineolyan/tzio/v1/TzEnv");

  let mut definition_methods: Vec<class::PoolIdx> = vec![];
  for (i, node) in tree.iter().enumerate() {
    let pool_idx = create_node_definition_method(i, node, &mut class);
    definition_methods.push(pool_idx);
  //   let mut file = output_dir.clone();
  //   file.set_file_name(node.0.get_id());
  //   file.set_extension("class");

  //   java::create_node_file(node, file.as_path())?;
  }

  create_constructor(&mut class, &definition_methods);

  writer::write(&class, output_file)
    .map_err(|e| format!("Failed to write into file. Caused by {}", e))
}

fn create_node_definition_method(
    i: usize,
    node: &NodeBlock,
    class: &mut class::JavaClass) -> class::PoolIdx {
  0
}

fn create_constructor(class: &mut class::JavaClass, definition_methods: &Vec<class::PoolIdx>) {
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![]
  };

  let with_slots_idx = 0;
  let mut operations = vec![
    // Configure the slots
    constructs::Operation::aload_0,
    constructs::Operation::iconst_1,
    // Push the array of inputs
    // Push the array of outputs
    constructs::Operation::invokevirtual(with_slots_idx)
  ];
  for idx in definition_methods {
    // Call each definition private method
    operations.push(constructs::Operation::aload_0);
    operations.push(constructs::Operation::invokespecial(*idx));
  }

  let access: u16 = 
    (constants::MethodAccess::FINAL as u16) |
    (constants::MethodAccess::PUBLIC as u16);

  let idx = class.create_method(
    access,
    &"<init>",
    signature,
    vec![
      constructs::Attribute::Code(3, operations)
    ]);
}