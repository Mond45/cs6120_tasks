use std::fs::File;

use bril_rs::{load_program, load_program_from_read};
use task6::{
    cfg::{form_cfg, get_basic_blocks, get_label},
    dom::{dom_frontier, find_dominators, rev_graph},
    ssa::{get_defs, place_phi_nodes},
};

fn main() {
    // let program = load_program();
    let program = load_program_from_read(File::open("loop.json").unwrap());

    for function in program.functions {
        println!("==== Function: {} ====", function.name);
        let blocks = get_basic_blocks(&function);

        let succ = form_cfg(&blocks);
        let pred = rev_graph(&succ);

        let dom = find_dominators(&pred, &succ);
        let df = dom_frontier(&dom, &pred);

        let defs = get_defs(&blocks);
        let blocks_phi_nodes = place_phi_nodes(&defs, &df, &pred);

        for (block, phi_nodes) in blocks_phi_nodes.iter().enumerate() {
            println!("Block: {}", get_label(&blocks, block));
            for (var, sources) in phi_nodes {
                println!(
                    "{var} -> {}",
                    sources
                        .iter()
                        .map(|&src| { get_label(&blocks, src) })
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            println!();
        }
        println!();
    }
}
