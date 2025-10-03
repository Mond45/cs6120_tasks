use std::collections::{HashMap, HashSet, VecDeque};

use bril_rs::{Code, Function, Instruction};

pub fn get_defs(blocks: &Vec<Vec<Code>>) -> HashMap<String, HashSet<usize>> {
    let mut defs: HashMap<String, HashSet<usize>> = HashMap::new();

    for (i, block) in blocks.iter().enumerate() {
        for code in block {
            if let Code::Instruction(
                Instruction::Constant { dest, .. } | Instruction::Value { dest, .. },
            ) = code
            {
                defs.entry(dest.clone()).or_default().insert(i);
            }
        }
    }

    defs
}

// block -> var -> labels
pub fn place_phi_nodes(
    defs: &HashMap<String, HashSet<usize>>,
    df: &Vec<Vec<usize>>,
    pred: &Vec<Vec<usize>>,
) -> Vec<HashMap<String, HashSet<usize>>> {
    // ref: https://pages.cs.wisc.edu/~fischer/cs701/lectures/Lecture25.4up.pdf
    let n = df.len();
    let mut phi_nodes: Vec<HashMap<String, HashSet<usize>>> = vec![HashMap::new(); n];

    for (var, def_blocks) in defs {
        let mut added = vec![false; n];

        // worklist of blocks to place phi nodes
        let mut worklist = VecDeque::new();
        for &block in def_blocks {
            added[block] = true;
            worklist.push_back(block);
        }

        while let Some(block) = worklist.pop_front() {
            for &df_block in &df[block] {
                // TODO: create a phi node consisting of blocks whose definition reaches `block`
                if !added[df_block] {
                    added[df_block] = true;
                    worklist.push_back(df_block);
                }
            }
        }
    }

    phi_nodes
}
