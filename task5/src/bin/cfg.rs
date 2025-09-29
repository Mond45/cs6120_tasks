use bril_rs::load_program;
use task5::{form_cfg, get_basic_blocks, get_label};

fn main() {
    let program = load_program();

    for function in program.functions {
        let blocks = get_basic_blocks(&function);
        let succ = form_cfg(&blocks);

        println!("digraph {{");
        for (u, vs) in succ.iter().enumerate() {
            for v in vs {
                println!(
                    "\t\"{}: {}\" -> \"{}: {}\"",
                    u,
                    get_label(&blocks, u),
                    v,
                    get_label(&blocks, *v)
                );
            }
        }
        println!("}}");
    }
}
