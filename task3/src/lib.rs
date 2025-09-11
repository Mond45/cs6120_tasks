use std::collections::{HashMap, HashSet};

use bril_rs::{Code, EffectOps, Function, Instruction};

fn flatten(blocks: Vec<Vec<Code>>) -> Vec<Code> {
    let mut instrs = Vec::new();
    for block in blocks {
        instrs.extend(block);
    }
    instrs
}

fn locally_killed_block(block: &mut Vec<Code>) -> bool {
    let mut changing = false;

    let mut drop_indices = Vec::new();
    // var -> index
    let mut last_assignment = HashMap::new();

    for (i, code) in block.iter().enumerate() {
        if let Code::Instruction(instr) = code {
            match instr {
                // drop last assignment
                Instruction::Constant { dest, .. } => {
                    if let Some(prev_index) = last_assignment.insert(dest.clone(), i) {
                        drop_indices.push(prev_index)
                    }
                }
                // commit last assignment
                Instruction::Effect { args, .. } => {
                    for arg in args.iter() {
                        last_assignment.remove(arg);
                    }
                }
                // can either drop / commit
                Instruction::Value { args, dest, .. } => {
                    for arg in args.iter() {
                        last_assignment.remove(arg);
                    }
                    if let Some(prev_index) = last_assignment.insert(dest.clone(), i) {
                        drop_indices.push(prev_index)
                    }
                }
            }
        }
    }

    let mut i: usize = 0;
    block.retain(|_| {
        let keep = !drop_indices.contains(&&i);
        if !keep {
            changing = true;
        }
        i += 1;
        keep
    });

    changing
}

pub fn locally_killed_pass(function: &mut Function) -> bool {
    let mut changing = false;

    let mut blocks = get_basic_blocks(&function);
    for block in blocks.iter_mut() {
        changing |= locally_killed_block(block);
    }

    function.instrs = flatten(blocks);

    changing
}

pub fn global_dce_pass(function: &mut Function) -> bool {
    let mut changing = false;
    let mut used = HashSet::new();

    for code in function.instrs.iter() {
        if let Code::Instruction(instr) = code {
            if let Instruction::Value { args, .. } | Instruction::Effect { args, .. } = instr {
                used.extend(args.iter().cloned());
            }
        }
    }

    function.instrs.retain(|code| {
        if let Code::Instruction(instr) = code {
            if let Instruction::Constant { dest, .. } | Instruction::Value { dest, .. } = instr {
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
    println!("```");
    for code in block {
        println!("{code}");
    }
    println!("```");
}
