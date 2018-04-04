mod class;
mod writer;
mod constants;

use std::path::Path;

use parser::ParsingTree;
use parser::syntax::{NodeBlock};

pub fn create_node_file(node_block: &NodeBlock, output_file: &Path) -> Result<(), String> {
  let mut class = class::JavaClass::new();

  writer::write(&class, output_file)
    .map_err(|e| format!("Failed to write into file. Caused by {}", e))
}

pub fn create_main_file(tree: &ParsingTree, output_file: &Path) -> Result<(), String> {
  let mut class = class::JavaClass::new();

  let mut classname = String::from("com/kineolyan/tzio/");
  classname.push_str(output_file.file_stem().unwrap().to_str().unwrap());
  classname.push_str("/Main");
  class.set_class(&classname);

  class.set_super_class(&"com/kineolyan/tzio/v1/TzEnv");
  
  writer::write(&class, output_file)
    .map_err(|e| format!("Failed to write into file. Caused by {}", e))
}