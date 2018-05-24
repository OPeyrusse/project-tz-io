mod dictionary;
mod class;
mod writer;
mod constants;
mod constructs;

use std::cmp::Eq;
use std::path::PathBuf;
use std::collections::HashMap;

use parser::ParsingTree;
use parser::syntax::{NodeBlock};
use generator::java::dictionary::Dictionary;

const TZ_ENV_CLASS_NAME: &str = "com/kineolyan/tzio/v1/TzEnv";

type SlotIndex = HashMap<(usize, usize), u32>;
#[derive(Debug, PartialEq, Hash)]
struct NodeSlot(String, u32, String, u32);
impl Eq for NodeSlot {}
struct SlotStructure {
  count: u32,
  node_inputs: SlotIndex,
  node_outputs: SlotIndex,
  input_indexes: Vec<u32>,
  output_indexes: Vec<u32>
}

fn create_slot_indexes(tree: &ParsingTree) -> SlotStructure {
  let mut input_index = HashMap::new();
  let mut output_index = HashMap::new();
  let mut slots: Dictionary<NodeSlot> = Dictionary::new();

  for (i, node) in tree.iter().enumerate() {
    let node_name = node.0.get_id();
    for (j, input) in node.1.iter().enumerate() {
      // FIXME Cannot work for node reading the inputs
      let node_slot = NodeSlot(
        input.from.node.get_id().clone(),
        input.from.port,
        node_name.clone(),
        input.to);
      let dic_idx = slots.map(node_slot);
      input_index.insert((i, j), dic_idx as u32);
    }
    for (k, output) in node.2.iter().enumerate() {
      // FIXME Cannot work for node writing to outputs
      let node_slot = NodeSlot(
        node_name.clone(),
        output.from,
        output.to.node.get_id().clone(),
        output.to.port);
      let dic_idx = slots.map(node_slot);
      output_index.insert((i, k), dic_idx as u32);
    }
  }

  SlotStructure {
    count: slots.size() as u32,
    node_inputs: input_index, 
    node_outputs: output_index,
    input_indexes: vec![],
    output_indexes: vec![]
  }
}

fn create_int_array(
    class: &mut class::JavaClass,
    values: &Vec<u32>, 
    var_idx: u8) -> constructs::Attribute {
  let array_size = class.map_integer(values.len() as u32);
  let mut operations = vec![
    constructs::Operation::ldc(array_size),
    constructs::Operation::newarray(constants::ArrayType::INT),
    constructs::Operation::astore(var_idx)
  ];
  for (i, value) in values.iter().enumerate() {
    let value_idx = class.map_integer(*value);
    let index_idx = class.map_integer(i as u32);

    // Add value to array
    operations.push(constructs::Operation::aload(var_idx));
    operations.push(constructs::Operation::ldc(index_idx));
    operations.push(constructs::Operation::ldc(value_idx));
    operations.push(constructs::Operation::iastore);
  }

  constructs::Attribute::Code(3, operations)
}

pub fn create_main_file(
    tree: &ParsingTree, 
    package: &str,
    output_dir: &PathBuf) -> Result<(), String> {
  let slots = create_slot_indexes(tree);
  let mut class = class::JavaClass::new();

  let mut classname = String::from("com/kineolyan/tzio/");
  classname.push_str(package);
  classname.push_str("/Main");
  class.set_class(&classname);

  class.set_super_class(&TZ_ENV_CLASS_NAME);

  let mut definition_methods: Vec<class::PoolIdx> = vec![];
  for (i, node) in tree.iter().enumerate() {
    let pool_idx = create_node_definition_method(i, node, &mut class);
    definition_methods.push(pool_idx);
  }

  create_constructor(&mut class, &definition_methods, &slots);
  create_main(&mut class);

  let mut output_file = output_dir.clone();
  output_file.push("Main");
  output_file.set_extension("class");
  writer::write(&class, output_file.as_path())
    .map_err(|e| format!("Failed to write into file. Caused by {}", e))
}

fn create_node_definition_method(
    i: usize,
    node: &NodeBlock,
    class: &mut class::JavaClass) -> class::PoolIdx {
  let add_node_idx = class.map_method(
    &TZ_ENV_CLASS_NAME, 
    "addNode", 
    &constructs::Signature {
      return_type: constants::Type::Object(String::from(TZ_ENV_CLASS_NAME)),
      parameter_types: vec![
        constants::Type::Object(String::from("java/lang/String")),
        constants::Type::Integer,
        constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
        constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
        constants::Type::Object(String::from("java/util/List"))
      ]
    });
  
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![]
  };
  
  let node_name = class.map_utf8_value(&node.0.get_id());
  let create_input_array = create_int_array(class, &vec![0, 1], 1);
  let create_output_array = create_int_array(class, &vec![1, 2], 2);
  let call_to_add_node = vec![
    constructs::Operation::aload(0),
    constructs::Operation::ldc(node_name),
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

fn create_constructor(
    class: &mut class::JavaClass, 
    definition_methods: &Vec<class::PoolIdx>,
    slots: &SlotStructure) -> class::PoolIdx {
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![]
  };

  let create_input_array_op = create_int_array(class, &slots.input_indexes, 1);
  let create_output_array_op = create_int_array(class, &slots.output_indexes, 2);

  let slot_count_cst = class.map_integer(slots.count);
  let with_slots_idx = get_with_slots_idx(class);
  let with_slots_op = vec![
    constructs::Operation::aload(0),
    constructs::Operation::ldc(slot_count_cst), // slot count
    constructs::Operation::iconst_1, // memory size
    constructs::Operation::aload(1), // inputs array
    constructs::Operation::aload(2), // output array
    constructs::Operation::invokevirtual(with_slots_idx)
  ];

  let mut create_nodes_op = Vec::new(); 
  for idx in definition_methods {
    // Call each definition private method
    create_nodes_op.push(constructs::Operation::aload(0));
    create_nodes_op.push(constructs::Operation::invokespecial(*idx));
  }

  let access: u16 = 
    (constants::MethodAccess::FINAL as u16) |
    (constants::MethodAccess::PUBLIC as u16);

  class.create_method(
    access,
    &"<init>",
    signature,
    vec![
      create_input_array_op,
      create_output_array_op,
      constructs::Attribute::Code(5, with_slots_op),
      constructs::Attribute::Code(1, create_nodes_op)
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
  let run_from_idx = get_run_from_idx(class);
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

fn get_with_slots_idx(class: &mut class::JavaClass) -> class::PoolIdx {
  let class_name: String;
  {
    class_name = class.get_class_name().expect("No class name set yet");
  }

  class.map_method(
    TZ_ENV_CLASS_NAME,
    &"withSlots", 
    &constructs::Signature {
      return_type: constants::Type::Object(class_name),
      parameter_types: vec![
        constants::Type::Integer,
        constants::Type::PrimitiveArray(
          1,
          constants::ArrayType::INT),
        constants::Type::PrimitiveArray(
          1,
          constants::ArrayType::INT)
      ]
    })
}

fn get_run_from_idx(class: &mut class::JavaClass) -> class::PoolIdx {
  class.map_method(
    &TZ_ENV_CLASS_NAME, 
    &"runFromSystem",
    &constructs::Signature {
      return_type: constants::Type::Void,
      parameter_types: vec![
        constants::Type::ObjectArray(1, String::from("java/lang/String"))
      ]
    })
}