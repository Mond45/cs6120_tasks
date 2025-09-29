use std::collections::HashMap;

use bril_rs::load_program;
use task5::{find_dominators, form_cfg, get_basic_blocks, get_label, rev_graph};

fn main() {
    let program = load_program();

    let mut dom_map = HashMap::new();

    for function in program.functions {
        let blocks = get_basic_blocks(&function);
        let succ = form_cfg(&blocks);
        let pred = rev_graph(&succ);

        let dom: HashMap<String, Vec<String>> = find_dominators(&pred, &succ)
            .into_iter()
            .enumerate()
            .map(|(a, doms)| {
                (
                    format!("{}: {}", a, get_label(&blocks, a)),
                    doms.into_iter()
                        .map(|d| format!("{}: {}", d, get_label(&blocks, d)))
                        .collect(),
                )
            })
            .collect();

        dom_map.insert(function.name, dom);
    }

    println!("{}", serde_json::to_string(&dom_map).unwrap());
}
