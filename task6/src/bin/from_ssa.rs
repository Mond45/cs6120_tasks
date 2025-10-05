use std::collections::HashMap;

use bril_rs::{Code, EffectOps, Instruction, Type, ValueOps, load_program, output_program};

fn get_ssa_types(instrs: &Vec<Code>) -> HashMap<String, Type> {
    let mut types = HashMap::new();
    for code in instrs {
        if let Code::Instruction(Instruction::Value {
            dest, op, op_type, ..
        }) = code
            && *op == ValueOps::Get
        {
            types.insert(dest.clone(), op_type.clone());
        }
    }
    types
}

fn main() {
    let mut program = load_program();

    for function in program.functions.iter_mut() {
        let mut instrs = Vec::new();

        let types = get_ssa_types(&function.instrs);

        for code in function.instrs.iter() {
            match code {
                Code::Instruction(Instruction::Value { op, .. }) if *op == ValueOps::Get => {}
                Code::Instruction(Instruction::Effect { op, args, .. })
                    if *op == EffectOps::Set =>
                {
                    if let [dest, src] = args.as_slice() {
                        instrs.push(Code::Instruction(Instruction::Value {
                            args: vec![src.clone()],
                            dest: dest.clone(),
                            funcs: vec![],
                            labels: vec![],
                            op: ValueOps::Id,
                            pos: None,
                            op_type: types[dest].clone(),
                        }))
                    } else {
                        panic!("invalid args for Set");
                    }
                }
                other => {
                    instrs.push(other.clone());
                }
            }
        }

        function.instrs = instrs;
    }

    output_program(&program);
}
