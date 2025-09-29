use bril_rs::load_program;
use task5::{display_dom, find_dominators, form_cfg, form_dom_tree, get_basic_blocks, rev_graph};

fn main() {
    let program = load_program();

    for function in program.functions {
        println!("function {}", function.name);

        let blocks = get_basic_blocks(&function);
        let succ = form_cfg(&blocks);
        let pred = rev_graph(&succ);
        let dominators = find_dominators(&pred, &succ);

        let mut dom_tree = form_dom_tree(&dominators);

        display_dom(&blocks, &mut dom_tree);

        println!();
    }
}
