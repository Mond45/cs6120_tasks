use bril_rs::{Code, EffectOps, Function, Instruction};

pub mod dce;
pub mod lvn;

fn flatten(blocks: Vec<Vec<Code>>) -> Vec<Code> {
    let mut instrs = Vec::new();
    for block in blocks {
        instrs.extend(block);
    }
    instrs
}

pub fn get_basic_blocks(function: &Function) -> Vec<Vec<Code>> {
    let mut basic_blocks = Vec::new();

    let mut current_block: Vec<Code> = Vec::new();
    for code in function.instrs.iter() {
        match code {
            Code::Label { .. } => {
                if !current_block.is_empty() {
                    basic_blocks.push(current_block);
                }
                current_block = vec![code.clone()];
            }
            Code::Instruction(instr) => match instr {
                Instruction::Effect { op, .. }
                    if matches!(op, EffectOps::Jump | EffectOps::Branch | EffectOps::Return) =>
                {
                    current_block.push(code.clone());
                    basic_blocks.push(current_block);
                    current_block = Vec::new();
                }
                _ => {
                    current_block.push(code.clone());
                }
            },
        }
    }

    if !current_block.is_empty() {
        basic_blocks.push(current_block);
    }

    basic_blocks
}

pub fn print_block(block: &Vec<Code>) {
    for code in block {
        println!("{code}");
    }
}
