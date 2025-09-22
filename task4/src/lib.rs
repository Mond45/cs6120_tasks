use std::collections::{HashMap, HashSet};

use bril_rs::{Code, EffectOps, Function, Instruction};

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
    let mut pred: HashMap<usize, Vec<usize>> =
        (0..blocks.len()).into_iter().map(|k| (k, vec![])).collect();
    let mut succ: HashMap<usize, Vec<usize>> =
        (0..blocks.len()).into_iter().map(|k| (k, vec![])).collect();

    for (i, block) in blocks.iter().enumerate() {
        if let Code::Instruction(Instruction::Effect {
            op: EffectOps::Jump | EffectOps::Branch,
            labels: target_labels,
            ..
        }) = block.last().expect("block shouldn't be empty")
        {
            let target_block_ids = find_block_ids_with_labels(blocks, target_labels);
            succ.get_mut(&i)
                .expect("succ should contain i")
                .extend(&target_block_ids);

            for pred_id in target_block_ids {
                // pred.entry(pred_id).or_default().push(i);
                pred.get_mut(&pred_id)
                    .expect("pred should contain pred_id")
                    .push(i);
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
            succ.get_mut(&i).expect("succ should contain i").push(i + 1);
            pred.get_mut(&(i + 1))
                .expect("pred should contain i + 1")
                .push(i);
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

fn merge(ins: &Vec<&HashSet<String>>) -> HashSet<String> {
    let mut merged = HashSet::new();
    // TODO: implement merge: union
    for i in ins {
        merged.extend(i.iter().cloned());
    }
    merged
}

fn transfer(block: &Vec<Code>, out: &HashSet<String>) -> HashSet<String> {
    // TODO: implement transfer function
    // find used, killed variables, returns use union (out - kill)

    let mut used = HashSet::new();
    let mut def = HashSet::new();

    for code in block {
        if let Code::Instruction(instr) = code {
            match instr {
                Instruction::Constant { dest, .. } => {
                    def.insert(dest.clone());
                }
                Instruction::Effect { args, .. } => {
                    used.extend(args.iter().cloned());
                }
                Instruction::Value { args, dest, .. } => {
                    used.extend(args.iter().cloned());
                    def.insert(dest.clone());
                }
            }
        }
    }

    let out_minus_def: HashSet<_> = out.difference(&def).cloned().collect();
    used.union(&out_minus_def).cloned().collect()
}

pub fn workman(
    blocks: &Vec<Vec<Code>>,
    pred: &HashMap<usize, Vec<usize>>,
    succ: &HashMap<usize, Vec<usize>>,
) -> Vec<Vec<String>> {
    let mut in_: Vec<HashSet<String>> = vec![HashSet::new(); blocks.len()];
    let mut out: Vec<HashSet<String>> = vec![HashSet::new(); blocks.len()];
    let mut worklist: Vec<_> = (0..blocks.len()).collect();

    while !worklist.is_empty() {
        let b = worklist.pop().expect("worklist shouldn't be empty");

        let in_succs = succ
            .get(&b)
            .expect("b should be in succ")
            .iter()
            .map(|s| in_.get(*s).expect("s should be in in_"))
            .collect::<Vec<_>>();
        out[b] = merge(&in_succs);

        let in_b = transfer(
            blocks.get(b).expect("b should be in blocks"),
            out.get(b).expect("b should be in out"),
        );
        if in_[b] != in_b {
            worklist.extend(pred.get(&b).expect("b should be in pred").iter());
        }
        in_[b] = in_b;
    }

    in_.into_iter().map(|v| v.into_iter().collect()).collect()
}
