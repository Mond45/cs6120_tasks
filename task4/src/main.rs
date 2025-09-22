use bril_rs::load_program;
use task4::{form_cfg, get_basic_blocks, workman};

fn main() {
    let program = load_program();

    for function in program.functions.iter() {
        let fn_basic_blocks = get_basic_blocks(&function);

        let (pred, succ) = form_cfg(&fn_basic_blocks);
        let mut block_live_vars = workman(&fn_basic_blocks, &pred, &succ);

        println!("fn {}:", function.name);

        for (i, live_vars) in block_live_vars.iter_mut().enumerate() {
            live_vars.sort();
            println!("{i} -> {}", live_vars.join(", "));
        }
    }
}
