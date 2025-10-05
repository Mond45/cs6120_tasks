use std::collections::{HashMap, HashSet};

use bril_rs::{Code, Instruction};

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

pub struct ReachingDefs;

impl ReachingDefs {
    // merge = union
    fn merge(inputs: &Vec<&HashMap<String, HashSet<usize>>>) -> HashMap<String, HashSet<usize>> {
        let mut merged: HashMap<String, HashSet<usize>> = HashMap::new();
        for input in inputs {
            for (var, block_ids) in input.iter() {
                merged.entry(var.clone()).or_default().extend(block_ids);
            }
        }
        merged
    }

    // out = def U (in - kill)
    fn transfer(
        block: &Vec<Code>,
        block_id: usize,
        in_: &HashMap<String, HashSet<usize>>,
    ) -> HashMap<String, HashSet<usize>> {
        let mut out = in_.clone();

        let defs = get_defs(&block);

        // TODO: find way to deal with arguments
        //
        // if block_id == 0 {
        //     for arg in args {
        //         defs.push(arg.name.clone());
        //     }
        // }

        // in - kill
        // remove previous definitions that are overwritten in the current block (killed)
        out.retain(|var, _| !defs.contains(var));

        // out U (in - kill)
        for var in defs {
            out.entry(var).or_default().insert(block_id);
        }

        out
    }

    pub fn find(
        blocks: &Vec<Vec<Code>>,
        pred: &Vec<Vec<usize>>,
        succ: &Vec<Vec<usize>>,
    ) -> (
        Vec<HashMap<String, HashSet<usize>>>,
        Vec<HashMap<String, HashSet<usize>>>,
    ) {
        let mut in_: Vec<HashMap<String, HashSet<usize>>> = vec![HashMap::new(); blocks.len()];
        let mut out: Vec<HashMap<String, HashSet<usize>>> = vec![HashMap::new(); blocks.len()];

        let mut worklist: Vec<_> = (0..blocks.len()).collect();

        while let Some(b) = worklist.pop() {
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
        }

        (in_, out)
    }
}
