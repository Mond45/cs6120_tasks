use std::collections::{HashMap, HashSet, VecDeque};

use bril_rs::{Code, Instruction};

use crate::df::{DataFlowAnalysis, ReachingDefs};

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
    blocks: &Vec<Vec<Code>>,
    defs: &HashMap<String, HashSet<usize>>,
    df: &Vec<Vec<usize>>,
    pred: &Vec<Vec<usize>>,
    succ: &Vec<Vec<usize>>,
) -> Vec<HashMap<String, HashSet<usize>>> {
    // ref: https://pages.cs.wisc.edu/~fischer/cs701/lectures/Lecture25.4up.pdf
    let n = df.len();
    let mut phi_nodes: Vec<HashMap<String, HashSet<usize>>> = vec![HashMap::new(); n];

    let (reaching_defs, _) = ReachingDefs::find(&blocks, &pred, &succ);

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
                // TODO: construct a phi node for var with labels according to var's reaching
                // definitions in the df_block
                phi_nodes
                    .get_mut(df_block)
                    .expect("df_block should be in phi_nodes")
                    .insert(
                        var.clone(),
                        reaching_defs[df_block]
                            .get(var)
                            .expect("var should be in reaching_defs[df_block]")
                            .clone(),
                    );

                if !added[df_block] {
                    added[df_block] = true;
                    worklist.push_back(df_block);
                }
            }
        }
    }

    phi_nodes
}

//TODO: accept HashMap<str, stack<str>>
pub fn rename(blocks: &mut Vec<Vec<Code>>, block: usize, idom: &Vec<Vec<usize>>) {
    let mut new_block = Vec::new();
    // TODO: replace args with stack[old name]
    // replace dest with new name
    // push new name to stack[old name]
    // for s in block's succ make phi node read from stack[v]
    for code in &blocks[block] {
        if let Code::Instruction(instr) = code {
            match instr {
                Instruction::Constant { dest, .. } => {}
                Instruction::Value { args, dest, .. } => {}
                Instruction::Effect { args, .. } => {}
            }
        }
    }
    blocks[block] = new_block;
    for &b in &idom[block] {
        rename(blocks, b, idom);
    }
    // pop all names pushed onto the stack
}
