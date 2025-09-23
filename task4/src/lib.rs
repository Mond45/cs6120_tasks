use std::collections::HashMap;

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

pub trait DataFlowAnalysis {
    const FORWARD: bool;
    type State: Clone + PartialEq;

    fn merge(inputs: &Vec<&Self::State>) -> Self::State;
    fn transfer(block: &Vec<Code>, input: &Self::State) -> Self::State;
    fn inital_state() -> Self::State;
    fn workman(
        blocks: &Vec<Vec<Code>>,
        pred: &HashMap<usize, Vec<usize>>,
        succ: &HashMap<usize, Vec<usize>>,
    ) -> (Vec<Self::State>, Vec<Self::State>) {
        let mut in_: Vec<Self::State> = vec![Self::inital_state(); blocks.len()];
        let mut out: Vec<Self::State> = vec![Self::inital_state(); blocks.len()];
        let mut worklist: Vec<_> = (0..blocks.len()).collect();

        while !worklist.is_empty() {
            let b = worklist.pop().expect("worklist shouldn't be empty");

            if Self::FORWARD {
                let out_preds = pred
                    .get(&b)
                    .expect("b should be in pred")
                    .iter()
                    .map(|s| out.get(*s).expect("s should be in out"))
                    .collect::<Vec<_>>();
                in_[b] = Self::merge(&out_preds);

                let out_b = Self::transfer(
                    blocks.get(b).expect("b should be in blocks"),
                    in_.get(b).expect("b should be in in_"),
                );
                if out[b] != out_b {
                    worklist.extend(succ.get(&b).expect("b should be in succ"));
                }
                out[b] = out_b;
            } else {
                let in_succs = succ
                    .get(&b)
                    .expect("b should be in succ")
                    .iter()
                    .map(|s| in_.get(*s).expect("s should be in in_"))
                    .collect::<Vec<_>>();
                out[b] = Self::merge(&in_succs);

                let in_b = Self::transfer(
                    blocks.get(b).expect("b should be in blocks"),
                    out.get(b).expect("b should be in out"),
                );
                if in_[b] != in_b {
                    worklist.extend(pred.get(&b).expect("b should be in pred"));
                }
                in_[b] = in_b;
            }
        }

        (in_, out)
    }
}

pub mod live_variables;
