// use std::fs::File;

use std::{collections::HashMap, env::args};

// use bril_rs::load_program_from_read;
use bril_rs::{Code, Instruction, ValueOps, load_program, output_program};
use task6::ssa::get_vars;
use task6::{
    cfg::{form_cfg, get_basic_blocks, get_label},
    dom::{dom_frontier, find_dominators, form_dom_tree, rev_graph},
    flatten,
    ssa::{PhiNodes, Renamer, get_defs, place_phi_nodes, ssa_rename},
};

fn display_dom(blocks: &Vec<Vec<Code>>, dom: &mut Vec<Vec<usize>>) {
    for (i, d) in dom.iter_mut().enumerate() {
        d.sort();
        eprintln!(
            "{i}: {} {:?}",
            get_label(&blocks, i),
            d.iter()
                .map(|idx| { format!("{idx}: {}", get_label(&blocks, *idx)) })
                .collect::<Vec<_>>()
        );
    }
}

fn display_phi_nodes(blocks: &Vec<Vec<Code>>, phi_nodes: &PhiNodes) {
    for (block_id, phi_nodes) in phi_nodes.iter().enumerate() {
        eprintln!("=== {block_id}: {} ===", get_label(&blocks, block_id));
        for (var, _) in phi_nodes {
            eprintln!("{var}",)
        }
    }
}

fn main() {
    let debug = args().any(|arg| arg == "-D");

    let mut program = load_program();

    for function in program.functions.iter_mut() {
        let mut stack: HashMap<String, Vec<String>> = HashMap::new();

        for arg in &function.args {
            stack
                .entry(arg.name.clone())
                .or_default()
                .push(arg.name.clone());
        }

        let vars = get_vars(&function.instrs);
        for (var, var_type) in vars {
            function.instrs.insert(
                0,
                Code::Instruction(Instruction::Value {
                    args: vec![],
                    dest: var.clone(),
                    funcs: vec![],
                    labels: vec![],
                    op: ValueOps::Undef,
                    pos: None,
                    op_type: var_type,
                }),
            );
            stack.entry(var.clone()).or_default().push(var);
        }

        let mut blocks = get_basic_blocks(&function);

        let succ = form_cfg(&blocks);
        let pred = rev_graph(&succ);

        let dom = find_dominators(&pred, &succ);
        let mut df = dom_frontier(&dom, &pred);
        if debug {
            eprintln!("dom frontier:");
            display_dom(&blocks, &mut df);
            eprintln!();
        }

        let mut idom = form_dom_tree(&dom);
        if debug {
            eprintln!("idom:");
            display_dom(&blocks, &mut idom);
            eprintln!();
        }

        let defs = get_defs(&blocks);
        let mut phi_nodes = place_phi_nodes(&defs, &df);

        if debug {
            eprintln!("Placed phi nodes:");
            display_phi_nodes(&blocks, &phi_nodes);
            eprintln!();
        }

        let mut renamer = Renamer::new();
        let mut phi_node_dests = HashMap::new();
        ssa_rename(
            0,
            &mut blocks,
            &mut phi_nodes,
            &idom,
            &mut renamer,
            &mut phi_node_dests,
            &mut stack,
            &succ,
        );

        if debug {
            eprintln!("Renamed phi nodes:");
            display_phi_nodes(&blocks, &phi_nodes);
            eprintln!();
        }

        function.instrs = flatten(blocks);
    }

    output_program(&program);
}
