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
        let mut in_ = out.clone();

        for code in block.iter().rev() {
            if let Code::Instruction(instr) = code {
                match instr {
                    Instruction::Constant { dest, .. } => {
                        in_.remove(dest);
                    }
                    Instruction::Effect { args, .. } => {
                        in_.extend(args.iter().cloned());
                    }
                    Instruction::Value { args, dest, .. } => {
                        in_.remove(dest);
                        in_.extend(args.iter().cloned());
                    }
                }
            }
        }

        in_
    }

    fn inital_state() -> Self::State {
        HashSet::new()
    }
}
