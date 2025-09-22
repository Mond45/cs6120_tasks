use bril_rs::load_program;
use task4::{DataFlowAnalysis, form_cfg, get_basic_blocks, live_variables::LiveVariables};

fn main() {
    let program = load_program();

    for function in program.functions.iter() {
        let fn_basic_blocks = get_basic_blocks(&function);

        let (pred, succ) = form_cfg(&fn_basic_blocks);
        let block_live_vars = LiveVariables::workman(&fn_basic_blocks, &pred, &succ);

        println!("fn {}:", function.name);

        for (i, live_vars) in block_live_vars.into_iter().enumerate() {
            let mut live_vars: Vec<_> = live_vars.into_iter().collect();
            live_vars.sort();
            println!("{i} -> {}", live_vars.join(", "));
        }
    }
}
