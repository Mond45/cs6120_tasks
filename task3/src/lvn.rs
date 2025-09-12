use std::collections::HashMap;

use bril_rs::{Code, Instruction, Literal, ValueOps};

#[derive(PartialEq)]
enum Value {
    Const(Literal),
    ValueOp(ValueOps, Vec<usize>),
}

pub fn lvn_pass(block: &mut Vec<Code>) -> bool {
    let mut changing = false;
    let mut table: Vec<(Value, String)> = Vec::new();
    let mut var2num: HashMap<String, usize> = HashMap::new();

    for code in block.iter_mut() {
        if let Code::Instruction(instr) = code {
            match instr {
                Instruction::Constant {
                    dest,
                    const_type,
                    value,
                    ..
                } => {
                    let instr_value = Value::Const(value.clone());

                    // there already exists a constant with the same value
                    if let Some(idx) = table.iter().position(|(value, _)| *value == instr_value) {
                        let (_, canonical_var) = &table[idx];
                        let cloned_dest = dest.clone();

                        *code = Code::Instruction(Instruction::Value {
                            args: vec![canonical_var.clone()],
                            dest: dest.clone(),
                            funcs: Vec::new(),
                            labels: Vec::new(),
                            op: ValueOps::Id,
                            pos: None,
                            op_type: const_type.clone(),
                        });

                        changing = true;

                        var2num.insert(cloned_dest, idx);
                    } else {
                    }
                }
                Instruction::Value { args, dest, op, .. } => {
                    let instr_value = Value::ValueOp(
                        *op,
                        args.iter().map(|arg| *var2num.get(arg).unwrap()).collect(),
                    );

                    if let Some(idx) = table.iter().position(|(value, _)| *value == instr_value) {
                    } else {
                    }
                }
                Instruction::Effect { args, op, .. } => {}
            }
        }
    }

    changing
}
