mod mapping;

use parser::ParsingResult;

pub fn process(parsingTree: &mut ParsingResult) {
  mapping::complete_mappings(parsingTree);
}