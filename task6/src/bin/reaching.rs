// use std::fs::File;

use bril_rs::load_program;
// use bril_rs::load_program_from_read;
use task6::{
    cfg::{form_cfg, get_basic_blocks, get_label},
    df::{DataFlowAnalysis, ReachingDefs},
    dom::rev_graph,
};

fn main() {
    let program = load_program();
    // let program = load_program_from_read(File::open("loop.json").unwrap());

    for function in program.functions {
        println!("==== Function: {} ====", function.name);
        let blocks = get_basic_blocks(&function);

        let succ = form_cfg(&blocks);
        let pred = rev_graph(&succ);

        let (in_, out) = ReachingDefs::find(&blocks, &pred, &succ);

        // block -> var
        println!("===IN===");
        for (block, defs) in in_.iter().enumerate() {
            println!("- .{}", get_label(&blocks, block));
            for (var, def_blocks) in defs.iter() {
                let mut def_blocks = def_blocks.iter().collect::<Vec<_>>();
                def_blocks.sort();
                println!(
                    "\t{var} -> {}",
                    def_blocks
                        .into_iter()
                        .map(|&b| { get_label(&blocks, b) })
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            println!();
        }

        println!("===OUT===");
        for (block, defs) in out.iter().enumerate() {
            println!("- .{}", get_label(&blocks, block));
            for (var, def_blocks) in defs.iter() {
                let mut def_blocks = def_blocks.iter().collect::<Vec<_>>();
                def_blocks.sort();
                println!(
                    "\t{var} -> {}",
                    def_blocks
                        .into_iter()
                        .map(|&b| { get_label(&blocks, b) })
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            println!();
        }
    }
}
