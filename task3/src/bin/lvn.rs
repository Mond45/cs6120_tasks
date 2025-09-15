use std::env::args;

use bril_rs::{load_program, output_program};
use task3::{flatten, get_basic_blocks, lvn::lvn};

fn main() {
    let mut program = load_program();

    let constant_folding = args().any(|arg| arg == "-f");

    for function in program.functions.iter_mut() {
        let mut basic_blocks = get_basic_blocks(function);

        for block in basic_blocks.iter_mut() {
            lvn(block, constant_folding);
        }

        function.instrs = flatten(basic_blocks);
    }

    output_program(&program);
}
