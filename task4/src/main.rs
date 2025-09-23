use bril_rs::load_program;
use task4::{DataFlowAnalysis, form_cfg, get_basic_blocks, live_variables::LiveVariables};

fn main() {
    let program = load_program();

    for function in program.functions.iter() {
        let fn_basic_blocks = get_basic_blocks(&function);

        let (pred, succ) = form_cfg(&fn_basic_blocks);
        let (in_, out) = LiveVariables::workman(&fn_basic_blocks, &pred, &succ);

        println!("fn {}:", function.name);

        for (i, (b_in, b_out)) in in_.into_iter().zip(out.into_iter()).enumerate() {
            let mut b_in: Vec<_> = b_in.into_iter().collect();
            let mut b_out: Vec<_> = b_out.into_iter().collect();
            b_in.sort();
            b_out.sort();
            println!(
                "{i}\n\tin: {}\n\tout: {}",
                b_in.join(", "),
                b_out.join(", ")
            );
        }
    }
}
