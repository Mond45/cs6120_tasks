use bril_rs::{Code, EffectOps, Function, Instruction};

pub fn get_label(blocks: &Vec<Vec<Code>>, idx: usize) -> String {
    if idx == 0
        && !matches!(
            blocks[idx].first().expect("block shouldn't be empty"),
            Code::Label { .. }
        )
    {
        return "entry".to_string();
    }

    match blocks[idx].first().expect("block shouldn't be empty") {
        Code::Label { label, .. } => label.clone(),
        Code::Instruction(instr) => instr.to_string(),
    }
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

pub fn form_cfg(blocks: &Vec<Vec<Code>>) -> Vec<Vec<usize>> {
    let mut succ: Vec<Vec<usize>> = vec![vec![]; blocks.len()];

    for (i, block) in blocks.iter().enumerate() {
        if let Code::Instruction(Instruction::Effect {
            op: EffectOps::Jump | EffectOps::Branch,
            labels: target_labels,
            ..
        }) = block.last().expect("block shouldn't be empty")
        {
            let target_block_ids = find_block_ids_with_labels(blocks, target_labels);
            succ[i].extend(&target_block_ids);
        } else if i < blocks.len() - 1
            && !matches!(
                block.last().expect("block shouldn't be empty"),
                Code::Instruction(Instruction::Effect {
                    op: EffectOps::Return,
                    ..
                })
            )
        {
            succ[i].push(i + 1);
        }
    }

    succ
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
