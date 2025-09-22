use std::{collections::HashMap, fs::File};

use bril_rs::{Code, Function, load_program, load_program_from_read};
use task4::{form_cfg, get_basic_blocks};

fn print_cfg(
    cfg: HashMap<usize, Vec<usize>>,
    fn_basic_blocks: &Vec<Vec<Code>>,
    function: &Function,
) {
    let mut cfg = cfg.into_iter().collect::<Vec<_>>();
    cfg.sort();

    println!("fn {}", function.name);
    for (i, next) in cfg.iter_mut() {
        next.sort();
        println!(
            "{} -> [{}]",
            fn_basic_blocks[*i].first().unwrap().to_string(),
            next.iter()
                .map(|e| fn_basic_blocks[*e].first().unwrap().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn main() {
    // let program = load_program();
    let program = load_program_from_read(File::open("lis.json").unwrap());

    for function in program.functions.iter() {
        let fn_basic_blocks = get_basic_blocks(&function);
        println!("pred:");
        let (mut pred, mut succ) = form_cfg(&fn_basic_blocks);
        print_cfg(pred, &fn_basic_blocks, function);
        println!("\nsucc:");
        print_cfg(succ, &fn_basic_blocks, function);
        println!("");
    }
}
