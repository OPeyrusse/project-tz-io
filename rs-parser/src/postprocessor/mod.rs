mod mapping;

use parser::ParsingTree;

pub fn process(parsingTree: &mut ParsingTree) {
  mapping::complete_mappings(parsingTree);
}