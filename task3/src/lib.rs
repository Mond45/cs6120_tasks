use std::collections::{HashMap, HashSet};

use bril_rs::{Code, EffectOps, Function, Instruction};

pub fn local_dce_pass(block: &Vec<Code>) -> bool {
    let mut changing = false;

    // let mut drop_idx = Vec::new();
    // let mut last_assignment = HashMap::new();
    //
    // for (i, &code) in block.iter().enumerate() {
    //     if let Code::Instruction(instr) = code {}
    // }

    changing
}

pub fn global_dce_pass(function: &mut Function) -> bool {
    let mut changing = false;
    let mut used = HashSet::new();

    for code in function.instrs.iter() {
        if let Code::Instruction(instr) = code {
            if let bril_rs::Instruction::Value { args, .. }
            | bril_rs::Instruction::Effect { args, .. } = instr
            {
                used.extend(args.iter().cloned());
            }
        }
    }

    function.instrs.retain(|code| {
        if let Code::Instruction(instr) = code {
            if let bril_rs::Instruction::Constant { dest, .. }
            | bril_rs::Instruction::Value { dest, .. } = instr
            {
                if used.contains(dest) {
                    return true;
                } else {
                    changing = true;
                    return false;
                }
            }
        }
        return true;
    });

    changing
}

pub fn get_basic_blocks(instrs: &Vec<Code>) -> Vec<Vec<Code>> {
    let mut basic_blocks = Vec::new();

    let mut current_block: Vec<Code> = Vec::new();
    for code in instrs.iter() {
        match code {
            Code::Label { .. } => {
                if current_block.len() > 0 {
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

    basic_blocks
}
