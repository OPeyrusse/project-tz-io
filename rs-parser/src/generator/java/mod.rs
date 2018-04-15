mod class;
mod writer;
mod constants;
mod constructs;

use std::path::Path;

use parser::ParsingTree;
use parser::syntax::{NodeBlock};

const TZ_ENV_CLASS_NAME: &str = "com/kineolyan/tzio/v1/TzEnv";

// pub fn create_node_file(node_block: &NodeBlock, output_file: &Path) -> Result<(), String> {
//   let mut class = class::JavaClass::new();

//   writer::write(&class, output_file)
//     .map_err(|e| format!("Failed to write into file. Caused by {}", e))
// }

fn create_int_array(values: &Vec<u32>, var_idx: u8) -> constructs::Attribute {
  let mut operations = vec![
    // Push the array length to the stack
    constructs::Operation::newarray(constants::ArrayType::INT),
    constructs::Operation::astore(var_idx)
  ];
  for (i, value) in values.iter().enumerate() {
    // Add value to array
    operations.push(constructs::Operation::aload(var_idx));
    // TODO Push the index into the stack
    // TODO Push the value into the stack
    operations.push(constructs::Operation::aastore)
  }

  constructs::Attribute::Code(3, operations)
}

pub fn create_main_file(tree: &ParsingTree, output_file: &Path) -> Result<(), String> {
  let mut class = class::JavaClass::new();

  let mut classname = String::from("com/kineolyan/tzio/");
  classname.push_str(output_file.file_stem().unwrap().to_str().unwrap());
  classname.push_str("/Main");
  class.set_class(&classname);

  class.set_super_class(&TZ_ENV_CLASS_NAME);

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
  create_main(&mut class);

  writer::write(&class, output_file)
    .map_err(|e| format!("Failed to write into file. Caused by {}", e))
}

fn create_node_definition_method(
    i: usize,
    node: &NodeBlock,
    class: &mut class::JavaClass) -> class::PoolIdx {
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![]
  };
  
  let add_node_idx = 0; // TODO reference the addNode method
  let mut create_input_array = create_int_array(&vec![0, 1], 1);
  let mut create_output_array = create_int_array(&vec![1, 2], 2);
  let call_to_add_node = vec![
    constructs::Operation::aload(0),
    // Push the name of the node
    constructs::Operation::iconst_1,
    constructs::Operation::aload(1),
    constructs::Operation::aload(2),
    // Push the list of operations
    constructs::Operation::invokevirtual(add_node_idx)
  ];

  let access: u16 = 
    (constants::MethodAccess::FINAL as u16) |
    (constants::MethodAccess::PRIVATE as u16);

  let mut method_name = String::from("createNode");
  method_name.push_str(&(i as u32).to_string());

  class.create_method(
    access,
    &method_name,
    signature,
    vec![
      create_input_array,
      create_output_array,
      constructs::Attribute::Code(6, call_to_add_node)
    ])
}

fn create_constructor(class: &mut class::JavaClass, definition_methods: &Vec<class::PoolIdx>) -> class::PoolIdx {
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![]
  };

  let with_slots_idx = 0;
  let mut operations = vec![
    // Configure the slots
    constructs::Operation::aload(0),
    constructs::Operation::iconst_1,
    // Push the array of inputs
    // Push the array of outputs
    constructs::Operation::invokevirtual(with_slots_idx)
  ];
  for idx in definition_methods {
    // Call each definition private method
    operations.push(constructs::Operation::aload(0));
    operations.push(constructs::Operation::invokespecial(*idx));
  }

  let access: u16 = 
    (constants::MethodAccess::FINAL as u16) |
    (constants::MethodAccess::PUBLIC as u16);

  class.create_method(
    access,
    &"<init>",
    signature,
    vec![
      constructs::Attribute::Code(3, operations)
    ])
}

fn create_main(class: &mut class::JavaClass) -> class::PoolIdx {
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![
      constants::Type::ObjectArray(1, String::from("java/lang/String"))
    ]
  };

  let this_class_idx = class.class_id;
  let run_from_idx = class.map_method(
    &TZ_ENV_CLASS_NAME, 
    &"runFromSystem",
    &constructs::Signature {
      return_type: constants::Type::Void,
      parameter_types: vec![
        constants::Type::ObjectArray(1, String::from("java/lang/String"))
      ]
    });
  let operations = vec![
    // Create a new instance of this class
    constructs::Operation::new(this_class_idx),
    // Call 'runFromSystem' with main parameter array
    constructs::Operation::aload(0),
    constructs::Operation::invokevirtual(run_from_idx)
  ];

  let access: u16 = 
    (constants::MethodAccess::STATIC as u16) |
    (constants::MethodAccess::PUBLIC as u16);

  class.create_method(
    access,
    &"main",
    signature,
    vec![
      constructs::Attribute::Code(3, operations)
    ])
} 