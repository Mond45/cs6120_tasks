use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

use bril_rs::{Code, EffectOps, Instruction, Type, ValueOps};

pub fn get_defs(blocks: &Vec<Vec<Code>>) -> HashMap<String, (HashSet<usize>, Type)> {
    let mut defs: HashMap<String, (HashSet<usize>, Type)> = HashMap::new();

    for (i, block) in blocks.iter().enumerate() {
        for code in block {
            if let Code::Instruction(
                Instruction::Constant {
                    dest,
                    const_type: var_type,
                    ..
                }
                | Instruction::Value {
                    dest,
                    op_type: var_type,
                    ..
                },
            ) = code
            {
                defs.entry(dest.clone())
                    .and_modify(|v| {
                        v.0.insert(i);
                    })
                    .or_insert((HashSet::from([i]), var_type.clone()));
            }
        }
    }

    defs
}

// block -> var -> type
pub type PhiNodes = Vec<HashMap<String, (Type, bool)>>;

pub fn place_phi_nodes(
    defs: &HashMap<String, (HashSet<usize>, Type)>,
    df: &Vec<Vec<usize>>,
) -> PhiNodes {
    // ref: https://pages.cs.wisc.edu/~fischer/cs701/lectures/Lecture25.4up.pdf
    let n = df.len();

    let mut phi_nodes: PhiNodes = vec![HashMap::new(); n];

    for (var, (def_blocks, var_type)) in defs {
        let mut added = vec![false; n];
        let mut phi_node_added = vec![false; n];

        // worklist of blocks to place phi nodes
        let mut worklist = VecDeque::new();
        for &block in def_blocks {
            added[block] = true;
            worklist.push_back(block);
        }

        while let Some(block) = worklist.pop_front() {
            for &df_block in &df[block] {
                if !phi_node_added[df_block] {
                    phi_node_added[df_block] = true;
                    phi_nodes[df_block].insert(var.clone(), (var_type.clone(), false));
                }

                if !added[df_block] {
                    added[df_block] = true;
                    worklist.push_back(df_block);
                }
            }
        }
    }

    phi_nodes
}

pub struct Renamer {
    counter: HashMap<String, usize>,
}

impl Renamer {
    pub fn new() -> Renamer {
        Renamer {
            counter: HashMap::new(),
        }
    }
    fn get_name(&mut self, var_name: &str) -> String {
        let cnt = self.counter.entry(var_name.to_owned()).or_insert(0);
        let new_name = format!("{var_name}.{cnt}");
        *cnt += 1;
        new_name
    }
}

pub fn ssa_rename(
    block: usize,
    blocks: &mut Vec<Vec<Code>>,
    phi_nodes: &mut PhiNodes,
    idom: &Vec<Vec<usize>>,
    renamer: &mut Renamer,
    phi_node_dests: &mut HashMap<(usize, String), String>,
    stack: &mut HashMap<String, Vec<String>>,
    succs: &Vec<Vec<usize>>,
) {
    let original_len: HashMap<String, usize> =
        stack.iter().map(|(k, v)| (k.clone(), v.len())).collect();

    for code in blocks[block].iter_mut() {
        if let Code::Instruction(instr) = code {
            match instr {
                Instruction::Constant { dest, .. } => {
                    let old_name = dest.clone();
                    *dest = renamer.get_name(&old_name);
                    stack.entry(old_name).or_default().push(dest.clone());
                }
                Instruction::Value { args, dest, op, .. } if *op != ValueOps::Get => {
                    for arg in args.iter_mut() {
                        *arg = stack
                            .get(&arg.clone())
                            .expect("arg should be in stack")
                            .last()
                            .expect("stack shouldn't be empty")
                            .clone();
                    }
                    let old_name = dest.clone();
                    *dest = renamer.get_name(&old_name);
                    stack.entry(old_name).or_default().push(dest.clone());
                }
                Instruction::Value { dest, op, .. } if *op == ValueOps::Get => {
                    let old_name = dest.clone();
                    *dest = phi_node_dests[&(block, old_name.clone())].clone();
                    stack.entry(old_name).or_default().push(dest.clone());
                }
                Instruction::Effect { args, op, .. } if *op != EffectOps::Set => {
                    for arg in args.iter_mut() {
                        *arg = stack
                            .get(&arg.clone())
                            .expect("arg should be in stack")
                            .last()
                            .expect("stack shouldn't be empty")
                            .clone();
                    }
                }
                _ => {}
            }
        }
    }

    for &succ in succs[block].iter() {
        for (dest, (var_type, inserted)) in phi_nodes
            .get_mut(succ)
            .expect("succ should be in phi_nodes")
        {
            let new_dest = phi_node_dests
                .entry((succ, dest.clone()))
                .or_insert(renamer.get_name(&dest));

            // place `set` in the current block
            let insert_idx = blocks[block]
                .iter()
                .rposition(|code| match code {
                    Code::Instruction(Instruction::Effect { op, .. })
                        if *op == EffectOps::Jump
                            || *op == EffectOps::Branch
                            || *op == EffectOps::Return =>
                    {
                        false
                    }
                    _ => true,
                })
                .map_or(0, |v| v + 1);

            let cloned_dest = dest.clone();
            if !stack.contains_key(&cloned_dest) || stack.get(&cloned_dest).unwrap().is_empty() {
                let new_name = renamer.get_name(&cloned_dest);
                stack
                    .entry(dest.clone())
                    .or_default()
                    .push(new_name.clone());
                blocks[block].splice(
                    insert_idx..insert_idx,
                    [
                        Code::Instruction(Instruction::Value {
                            args: vec![],
                            dest: new_name,
                            funcs: vec![],
                            labels: vec![],
                            op: ValueOps::Undef,
                            pos: None,
                            op_type: var_type.clone(),
                        }),
                        Code::Instruction(Instruction::Effect {
                            args: vec![
                                new_dest.clone(),
                                stack
                                    .get(&cloned_dest)
                                    .expect("arg should be in stack")
                                    .last()
                                    .expect("stack shouldn't be empty")
                                    .clone(),
                            ],
                            funcs: vec![],
                            labels: vec![],
                            op: EffectOps::Set,
                            pos: None,
                        }),
                    ],
                );
            } else {
                blocks[block].insert(
                    insert_idx,
                    Code::Instruction(Instruction::Effect {
                        args: vec![
                            new_dest.clone(),
                            stack
                                .get(&cloned_dest)
                                .expect("arg should be in stack")
                                .last()
                                .expect("stack shouldn't be empty")
                                .clone(),
                        ],
                        funcs: vec![],
                        labels: vec![],
                        op: EffectOps::Set,
                        pos: None,
                    }),
                );
            }

            // place `get` in the successor
            if !*inserted {
                *inserted = true;

                let insert_idx = blocks[succ]
                    .iter()
                    .position(|code| !matches!(code, Code::Label { .. }))
                    .unwrap_or(1);
                blocks[succ].insert(
                    insert_idx,
                    Code::Instruction(Instruction::Value {
                        args: vec![],
                        dest: dest.clone(),
                        funcs: vec![],
                        labels: vec![],
                        op: ValueOps::Get,
                        pos: None,
                        op_type: var_type.clone(),
                    }),
                );
            }
        }
    }

    for &b in &idom[block] {
        ssa_rename(
            b,
            blocks,
            phi_nodes,
            idom,
            renamer,
            phi_node_dests,
            stack,
            succs,
        );
    }

    // restore stack to original len
    for (var, stk) in stack.iter_mut() {
        stk.truncate(*original_len.get(var).unwrap_or(&0));
    }
}
