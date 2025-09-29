use bril_rs::load_program;
use task5::{find_dominators, form_cfg, get_basic_blocks, get_label, rev_graph};

fn main() {
    let program = load_program();

    for function in program.functions {
        println!("function {}", function.name);

        let blocks = get_basic_blocks(&function);
        let succ = form_cfg(&blocks);
        let pred = rev_graph(&succ);
        let mut dom: Vec<Vec<usize>> = find_dominators(&pred, &succ)
            .into_iter()
            .map(|d| d.into_iter().collect())
            .collect();

        for (i, d) in dom.iter_mut().enumerate() {
            d.sort();
            println!(
                "{i}: {} {:?}",
                get_label(&blocks, i),
                d.iter()
                    .map(|idx| { format!("{idx}: {}", get_label(&blocks, *idx)) })
                    .collect::<Vec<_>>()
            );
        }

        println!();
    }
}
