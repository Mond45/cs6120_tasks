use std::collections::{HashMap, HashSet};

use bril_rs::{Code, Instruction};

pub trait DataFlowAnalysis {
    const FORWARD: bool;
    type State: Clone + PartialEq;

    fn merge(inputs: &Vec<&Self::State>) -> Self::State;
    fn transfer(block: &Vec<Code>, block_id: usize, input: &Self::State) -> Self::State;
    fn initial_state() -> Self::State;
    // returns in, out
    fn find(
        blocks: &Vec<Vec<Code>>,
        pred: &Vec<Vec<usize>>,
        succ: &Vec<Vec<usize>>,
    ) -> (Vec<Self::State>, Vec<Self::State>) {
        let mut in_: Vec<Self::State> = vec![Self::initial_state(); blocks.len()];
        let mut out: Vec<Self::State> = vec![Self::initial_state(); blocks.len()];
        let mut worklist: Vec<_> = (0..blocks.len()).collect();

        while let Some(b) = worklist.pop() {
            if Self::FORWARD {
                let out_preds = pred
                    .get(b)
                    .expect("b should be in pred")
                    .iter()
                    .map(|s| out.get(*s).expect("s should be in out"))
                    .collect::<Vec<_>>();
                in_[b] = Self::merge(&out_preds);

                let out_b = Self::transfer(
                    blocks.get(b).expect("b should be in blocks"),
                    b,
                    in_.get(b).expect("b should be in in_"),
                );
                if out[b] != out_b {
                    worklist.extend(succ.get(b).expect("b should be in succ"));
                }
                out[b] = out_b;
            } else {
                let in_succs = succ
                    .get(b)
                    .expect("b should be in succ")
                    .iter()
                    .map(|s| in_.get(*s).expect("s should be in in_"))
                    .collect::<Vec<_>>();
                out[b] = Self::merge(&in_succs);

                let in_b = Self::transfer(
                    blocks.get(b).expect("b should be in blocks"),
                    b,
                    out.get(b).expect("b should be in out"),
                );
                if in_[b] != in_b {
                    worklist.extend(pred.get(b).expect("b should be in pred"));
                }
                in_[b] = in_b;
            }
        }

        (in_, out)
    }
}

pub struct ReachingDefs;

fn get_defs(block: &Vec<Code>) -> Vec<String> {
    let mut defs = Vec::new();
    for code in block {
        if let Code::Instruction(
            Instruction::Constant { dest, .. } | Instruction::Value { dest, .. },
        ) = code
        {
            defs.push(dest.clone());
        }
    }
    defs
}

impl DataFlowAnalysis for ReachingDefs {
    const FORWARD: bool = true;

    // var -> block ids
    type State = HashMap<String, HashSet<usize>>;

    // merge = union
    fn merge(inputs: &Vec<&Self::State>) -> Self::State {
        let mut merged: Self::State = HashMap::new();
        for input in inputs {
            for (var, block_ids) in input.iter() {
                merged.entry(var.clone()).or_default().extend(block_ids);
            }
        }
        merged
    }

    // out = def U (in - kill)
    fn transfer(block: &Vec<Code>, block_id: usize, in_: &Self::State) -> Self::State {
        let mut out = in_.clone();

        let defs = get_defs(&block);

        // in - kill
        // remove previous definitions that are overwritten in the current block (killed)
        out.retain(|var, _| !defs.contains(var));

        // out U (in - kill)
        for var in defs {
            out.entry(var).or_default().insert(block_id);
        }

        out
    }

    fn initial_state() -> Self::State {
        HashMap::new()
    }
}
