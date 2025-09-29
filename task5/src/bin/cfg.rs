use std::collections::HashMap;
use std::fmt::Write;

use bril_rs::load_program;
use task5::{form_cfg, get_basic_blocks, get_label};

fn main() {
    let program = load_program();

    let mut cfg_map = HashMap::new();

    for function in program.functions {
        let blocks = get_basic_blocks(&function);
        let succ = form_cfg(&blocks);

        let mut dot = "digraph {\n".to_string();

        for (u, vs) in succ.iter().enumerate() {
            for v in vs {
                let _ = writeln!(
                    &mut dot,
                    "\t\"{}: {}\" -> \"{}: {}\"",
                    u,
                    get_label(&blocks, u),
                    v,
                    get_label(&blocks, *v)
                );
            }
        }
        let _ = writeln!(&mut dot, "}}");

        cfg_map.insert(function.name, dot);
    }

    println!("{}", serde_json::to_string(&cfg_map).unwrap());
}
