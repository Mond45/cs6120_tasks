use crate::{flatten, get_basic_blocks};
use std::collections::{HashMap, HashSet};

use bril_rs::{Code, Function, Instruction};

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
