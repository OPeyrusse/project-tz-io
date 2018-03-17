mod mapping;

use parser::ParsingTree;

pub fn process(tree: &mut ParsingTree) {
  mapping::complete_mappings(tree);
}