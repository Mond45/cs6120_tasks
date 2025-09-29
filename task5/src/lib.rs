use std::collections::HashSet;

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

fn postorder(u: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[u] = true;
    for v in graph[u].iter() {
        if !visited[*v] {
            postorder(*v, graph, visited, order);
        }
    }
    order.push(u);
}

pub fn find_dominators(preds: &Vec<Vec<usize>>, succs: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut dom: Vec<HashSet<usize>> = vec![(0..preds.len()).collect(); preds.len()];
    dom[0] = [0].into();

    /*
    dom = {every block -> all blocks}
    dom[entry] = {entry}
    while dom is still changing:
        for vertex in CFG except entry:
            dom[vertex] = {vertex} ∪ ⋂(dom[p] for p in vertex.preds}
    */

    let mut rev_postorder = vec![];
    let mut visited = vec![false; preds.len()];
    postorder(0, &succs, &mut visited, &mut rev_postorder);
    rev_postorder.pop();
    rev_postorder.reverse();

    let mut changing = true;
    while changing {
        changing = false;

        for v in rev_postorder.iter() {
            let mut new_dom: HashSet<_> = preds
                .get(*v)
                .expect("v should be in preds")
                .iter()
                .map(|pred| dom.get(*pred).expect("pred should be in dom"))
                .fold((0..preds.len()).collect(), |accum, val| {
                    accum.intersection(val).cloned().collect()
                });
            new_dom.insert(*v);

            changing |= dom.get(*v).unwrap() == &new_dom;
            dom[*v] = new_dom;
        }
    }

    dom.into_iter().map(|d| d.into_iter().collect()).collect()
}

pub fn rev_graph(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut output = vec![HashSet::new(); graph.len()];

    for (from, tos) in graph.iter().enumerate() {
        for &to in tos {
            output[to].insert(from);
        }
    }

    output
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect()
}

pub fn form_dom_tree(dominators: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let dominates: Vec<HashSet<_>> = rev_graph(&dominators)
        .into_iter()
        .map(|v| v.into_iter().collect::<HashSet<_>>())
        .collect();

    let n = dominators.len();

    let mut dom_tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    for a in 0..n {
        for b in 0..n {
            if a != b
                && dominates[a].contains(&b)
                && (0..n).into_iter().all(|c| {
                    if a == c || b == c {
                        true
                    } else {
                        !(dominates[a].contains(&c) && dominates[c].contains(&b))
                    }
                })
            {
                dom_tree[a].push(b);
            }
        }
        dom_tree[a].sort();
    }

    dom_tree
}

pub fn display_dom(blocks: &Vec<Vec<Code>>, dom: &mut Vec<Vec<usize>>) {
    for (i, d) in dom.iter_mut().enumerate() {
        d.sort();
        println!(
            "{i}: {} {:?}",
            get_label(&blocks, i),
            d.iter()
                .map(|idx| { format!("{idx}: {}", get_label(&blocks, *idx)) })
                .collect::<Vec<_>>()
        );
    }
}
