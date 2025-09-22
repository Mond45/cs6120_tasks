use std::collections::HashSet;

use bril_rs::{Code, Instruction};

use crate::DataFlowAnalysis;

pub struct LiveVariables;

impl DataFlowAnalysis for LiveVariables {
    const FORWARD: bool = false;

    type State = HashSet<String>;

    fn merge(inputs: &Vec<&Self::State>) -> Self::State {
        let mut merged = HashSet::new();
        for i in inputs {
            merged.extend(i.iter().cloned());
        }
        merged
    }

    fn transfer(block: &Vec<Code>, out: &Self::State) -> Self::State {
        let mut used = HashSet::new();
        let mut def = HashSet::new();

        for code in block {
            if let Code::Instruction(instr) = code {
                match instr {
                    Instruction::Constant { dest, .. } => {
                        def.insert(dest.clone());
                    }
                    Instruction::Effect { args, .. } => {
                        used.extend(args.iter().cloned());
                    }
                    Instruction::Value { args, dest, .. } => {
                        used.extend(args.iter().cloned());
                        def.insert(dest.clone());
                    }
                }
            }
        }

        let out_minus_def: HashSet<_> = out.difference(&def).cloned().collect();
        used.union(&out_minus_def).cloned().collect()
    }

    fn inital_state() -> Self::State {
        HashSet::new()
    }
}
