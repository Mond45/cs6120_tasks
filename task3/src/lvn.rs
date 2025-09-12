use std::{
    collections::{HashMap, HashSet},
    sync::atomic::AtomicUsize,
};

use bril_rs::{Code, Instruction, Literal, Type, ValueOps};

#[derive(PartialEq)]
enum Value {
    Const(Type, Literal),
    External,
    ValueOp(ValueOps, Vec<usize>),
}

fn can_reuse(op: ValueOps) -> bool {
    use ValueOps::*;
    match op {
        Call | Get | Alloc | Load | PtrAdd => false,
        _ => true,
    }
}

fn generate_var_name(original_name: &str) -> String {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    format!(
        "{}__{:0>9}",
        original_name,
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    )
}

pub fn get_external_vars(block: &[Code]) -> Vec<String> {
    let mut defs = HashSet::new();
    let mut externals = HashSet::new();

    for code in block {
        if let Code::Instruction(Instruction::Value { args, .. })
        | Code::Instruction(Instruction::Effect { args, .. }) = code
        {
            for arg in args {
                if !defs.contains(arg) {
                    externals.insert(arg.clone());
                }
            }
        }

        if let Code::Instruction(Instruction::Constant { dest, .. })
        | Code::Instruction(Instruction::Value { dest, .. }) = code
        {
            defs.insert(dest.clone());
        }
    }

    externals.into_iter().collect()
}

fn get_assignments(block: &[Code]) -> HashMap<String, Vec<usize>> {
    let mut assignments: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, code) in block.iter().enumerate() {
        if let Code::Instruction(Instruction::Constant { dest, .. })
        | Code::Instruction(Instruction::Value { dest, .. }) = code
        {
            assignments.entry(dest.clone()).or_default().push(i);
        }
    }

    assignments
}

pub fn lvn(block: &mut Vec<Code>) {
    let mut table: Vec<(Value, String)> = Vec::new();
    let mut var2idx: HashMap<String, usize> = HashMap::new();

    for var in get_external_vars(&block) {
        table.push((Value::External, var.clone()));
        var2idx.insert(var, table.len() - 1);
    }

    let assignments = get_assignments(&block);

    let mut updates = Vec::new();
    for (idx_instr, code) in block.iter().enumerate() {
        if let Code::Instruction(instr) = code {
            match instr {
                Instruction::Constant {
                    dest,
                    value,
                    const_type,
                    ..
                } => {
                    let instr_value = Value::Const(const_type.clone(), value.clone());

                    if let Some((idx_table, (_, var))) = table
                        .iter()
                        .enumerate()
                        .find(|(_, (val, _))| *val == instr_value)
                    {
                        updates.push((
                            idx_instr,
                            Code::Instruction(Instruction::Value {
                                args: vec![var.clone()],
                                dest: dest.clone(),
                                funcs: vec![],
                                labels: vec![],
                                op: ValueOps::Id,
                                pos: None,
                                op_type: const_type.clone(),
                            }),
                        ));

                        var2idx.insert(dest.clone(), idx_table);
                    } else {
                        let mut new_dest = dest.clone();

                        if let Some(var_assignments) = assignments.get(dest) {
                            if var_assignments.len() > 1
                                && idx_instr != *var_assignments.last().unwrap()
                            {
                                new_dest = generate_var_name(&dest);
                            }
                        }

                        let mut new_instr = code.clone();
                        if let Code::Instruction(Instruction::Constant { dest, .. }) =
                            &mut new_instr
                        {
                            *dest = new_dest.clone();
                        }

                        updates.push((idx_instr, new_instr));

                        table.push((instr_value, new_dest.clone()));

                        var2idx.insert(dest.clone(), table.len() - 1);
                    }
                }
                Instruction::Value {
                    args,
                    dest,
                    op,
                    op_type,
                    ..
                } => {
                    let instr_value = Value::ValueOp(
                        op.clone(),
                        args.iter()
                            .map(|arg| var2idx.get(arg).unwrap().clone())
                            .collect(),
                    );

                    if let Some((idx_table, (_, var))) = table
                        .iter()
                        .enumerate()
                        .find(|(_, (val, _))| *val == instr_value)
                        && can_reuse(*op)
                    {
                        updates.push((
                            idx_instr,
                            Code::Instruction(Instruction::Value {
                                args: vec![var.clone()],
                                dest: dest.clone(),
                                funcs: vec![],
                                labels: vec![],
                                op: ValueOps::Id,
                                pos: None,
                                op_type: op_type.clone(),
                            }),
                        ));

                        var2idx.insert(dest.clone(), idx_table);
                    } else {
                        let mut new_dest = dest.clone();

                        if let Some(var_assignments) = assignments.get(dest) {
                            if var_assignments.len() > 1
                                && idx_instr != *var_assignments.last().unwrap()
                            {
                                new_dest = generate_var_name(&dest);
                            }
                        }

                        let mut new_instr = code.clone();
                        if let Code::Instruction(Instruction::Value { dest, args, .. }) =
                            &mut new_instr
                        {
                            *args = args
                                .iter()
                                .map(|arg| table[*var2idx.get(arg).unwrap()].1.clone())
                                .collect();
                            *dest = new_dest.clone();
                        }

                        updates.push((idx_instr, new_instr));

                        table.push((instr_value, new_dest.clone()));

                        var2idx.insert(dest.clone(), table.len() - 1);
                    }
                }
                Instruction::Effect { .. } => {
                    let mut new_instr = code.clone();
                    if let Code::Instruction(Instruction::Effect { args, .. }) = &mut new_instr {
                        *args = args
                            .iter()
                            .map(|arg| table[*var2idx.get(arg).unwrap()].1.clone())
                            .collect();
                    }

                    updates.push((idx_instr, new_instr));
                }
            }
        }
    }

    for (i, code) in updates {
        block[i] = code;
    }
}
