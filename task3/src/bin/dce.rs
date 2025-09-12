use bril_rs::{load_program, output_program};
use task3::dce::{global_dce_pass, locally_killed_pass};

fn main() {
    let mut program = load_program();

    let mut changing = true;
    while changing {
        changing = false;
        for function in program.functions.iter_mut() {
            changing |= global_dce_pass(function);
            changing |= locally_killed_pass(function);
        }
    }

    output_program(&program);
}
