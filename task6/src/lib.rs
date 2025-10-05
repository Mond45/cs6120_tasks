use bril_rs::Code;

pub mod cfg;
pub mod df;
pub mod dom;
pub mod ssa;

pub fn flatten(blocks: Vec<Vec<Code>>) -> Vec<Code> {
    let mut instrs = Vec::new();
    for block in blocks {
        instrs.extend(block);
    }
    instrs
}
