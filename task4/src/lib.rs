use std::collections::{HashMap, VecDeque};

use bril_rs::{Code, EffectOps, Function, Instruction};

fn get_block_id(blocks: &Vec<Vec<Code>>, block: &Vec<Code>) -> usize {
    blocks
        .iter()
        .position(|blk| blk == block)
        .expect("block should be present in blocks")
}

fn find_block_ids_with_labels(blocks: &Vec<Vec<Code>>, labels: &Vec<String>) -> Vec<usize> {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| {
            if let Code::Label { label, .. } = block.first().expect("block shouldn't be empty")
                && labels.contains(label)
            {
                true
            } else {
                false
            }
        })
        .map(|(i, _)| i)
        .collect()
}

pub fn form_cfg(
    blocks: &Vec<Vec<Code>>,
) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut pred: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut succ: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, block) in blocks.iter().enumerate() {
        if let Code::Instruction(Instruction::Effect {
            op: EffectOps::Jump | EffectOps::Branch,
            labels: target_labels,
            ..
        }) = block.last().expect("block shouldn't be empty")
        {
            let target_block_ids = find_block_ids_with_labels(blocks, target_labels);
            succ.entry(i).or_default().extend(&target_block_ids);

            for pred_id in target_block_ids {
                pred.entry(pred_id).or_default().push(i);
            }
        } else if i < blocks.len() - 1
            && !matches!(
                block.last().expect("block shouldn't be empty"),
                Code::Instruction(Instruction::Effect {
                    op: EffectOps::Return,
                    ..
                })
            )
        {
            succ.entry(i).or_default().push(i + 1);
            pred.entry(i + 1).or_default().push(i);
        }
    }

    (pred, succ)
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

pub fn workman() {
    // let mut queue = VecDeque::new();
}
