use bril_rs::load_program;
use task4::{form_cfg, get_basic_blocks};

fn main() {
    let program = load_program();

    for function in program.functions.iter() {
        let fn_basic_blocks = get_basic_blocks(&function);
        let (mut pred, mut succ) = form_cfg(&fn_basic_blocks);

        println!("fn {}", function.name);
        for (i, next) in succ.iter_mut() {
            next.sort();
            println!(
                "{} -> [{}]",
                i,
                next.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}
