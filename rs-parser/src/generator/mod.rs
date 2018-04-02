use parser::ParsingTree;
use parser::address::Node;
use parser::syntax::NodeBlock;
use std::path::{Path, PathBuf};
use std::fs;

fn prepare_output<'a>(filename: &'a str, target_dir: &'a str) -> Result<PathBuf, String> {
	let base_name = Path::new(filename).file_stem().unwrap();
	let mut output_dir_buffer = PathBuf::from(target_dir);
  output_dir_buffer.push(base_name);
  let mut result: Result<(), String> = Ok(());
  { 
    let output_dir = output_dir_buffer.as_path();
    // Clean the existing directory
    let _deleted = fs::remove_dir_all(output_dir);
    let created = fs::create_dir_all(output_dir);
    if created.is_err() {
      result = Err(format!(
        "Could not create output directory {} due to error {}", 
        output_dir.to_str().unwrap(), created.unwrap_err()))
    }
  }

  result.map(|_| output_dir_buffer)
}

fn create_node_file(node_block: &NodeBlock, output_file: &Path) -> Result<(), String> {
  Ok(())
}

fn create_main_file(tree: &ParsingTree, output_file: &Path) -> Result<(), String> {
  Ok(())
}

fn generate_program(tree: &ParsingTree, output_dir: PathBuf) -> Result<(), String> {
  let mut main_file = output_dir.clone();
  main_file.push("Main.class");
  let mut result = create_main_file(&tree, main_file.as_path());
  for node in tree {
    let mut file = output_dir.clone();
    file.push(node.0.get_id());
    result = result.and_then(|_| create_node_file(node, file.as_path()));
  }
  result
}

pub fn generate(result: ParsingTree, filename: &str, target_dir: &str) -> Result<(), String> {
	prepare_output(filename, target_dir)
    .and_then(|output_dir| generate_program(&result, output_dir))
}