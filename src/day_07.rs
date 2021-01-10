use itertools::Itertools;

mod amp_stack {
    use crate::intcode::intcode::Intcode;
    use crate::intcode::intcode::IntcodeState;

    #[derive(Debug)]
    pub struct AmpStack {
        amp_a: Intcode,
        amp_b: Intcode,
        amp_c: Intcode,
        amp_d: Intcode,
        amp_e: Intcode,
    }
    impl AmpStack {
        pub fn new() -> AmpStack {
            AmpStack {
                amp_a: Intcode::new(),
                amp_b: Intcode::new(),
                amp_c: Intcode::new(),
                amp_d: Intcode::new(),
                amp_e: Intcode::new(),
            }
        }
        pub fn is_halted(&self) -> bool {
            self.amp_e.get_state() == IntcodeState::Halted
        }
        pub fn init_amps(&mut self, program: Vec<isize>, phases: Vec<isize>) {
            self.amp_a.load_program(program.clone());
            self.amp_b.load_program(program.clone());
            self.amp_c.load_program(program.clone());
            self.amp_d.load_program(program.clone());
            self.amp_e.load_program(program.clone());
            self.amp_a.set_input(phases.iter().nth(0).unwrap().clone());
            self.amp_b.set_input(phases.iter().nth(1).unwrap().clone());
            self.amp_c.set_input(phases.iter().nth(2).unwrap().clone());
            self.amp_d.set_input(phases.iter().nth(3).unwrap().clone());
            self.amp_e.set_input(phases.iter().nth(4).unwrap().clone());
            // Turn on amps, they will pause for the next input
            self.amp_a.run();
            self.amp_b.run();
            self.amp_c.run();
            self.amp_d.run();
            self.amp_e.run();
        }
        pub fn process(&mut self, signal_input: isize) -> isize {
            // Daisy-chain inputs and ouputs, start another processing cycle
            self.amp_a.set_input(signal_input);
            self.amp_a.run();
            self.amp_b.set_input(self.amp_a.read_output(0));
            self.amp_b.run();
            self.amp_c.set_input(self.amp_b.read_output(0));
            self.amp_c.run();
            self.amp_d.set_input(self.amp_c.read_output(0));
            self.amp_d.run();
            self.amp_e.set_input(self.amp_d.read_output(0));
            self.amp_e.run();
            let result = self.amp_e.read_output(0);
            // Flush outputs
            self.amp_a.flush_output();
            self.amp_b.flush_output();
            self.amp_c.flush_output();
            self.amp_d.flush_output();
            self.amp_e.flush_output();
            result
        }
    }
}
fn tune_amps_part_1(program: &Vec<isize>) -> isize {
    let mut outputs = Vec::new();
    let signal_input = 0;
    let phases = vec![4, 3, 2, 1, 0]
        .into_iter()
        .permutations(5)
        .collect::<Vec<_>>();

    for combo in phases {
        let clone = program.clone();
        let mut amp_stack = amp_stack::AmpStack::new();
        amp_stack.init_amps(clone, combo);
        outputs.push(amp_stack.process(signal_input));
    }
    *outputs.iter().max().unwrap()
}

fn tune_amps_part_2(program: &Vec<isize>) -> isize {
    let mut outputs = Vec::new();
    let signal_input = 0;
    let phases = vec![9, 8, 7, 6, 5]
        .into_iter()
        .permutations(5)
        .collect::<Vec<_>>();

    for combo in phases {
        let clone = program.clone();
        let mut amp_stack = amp_stack::AmpStack::new();
        amp_stack.init_amps(clone, combo);
        // Infinite loop avoider for testing
        let max_runs = 1000;
        let mut output = amp_stack.process(signal_input); // Initial run
        for _ in 0..max_runs {
            output = amp_stack.process(output);
            if amp_stack.is_halted() {
                outputs.push(output.clone());
                break;
            }
        }
    }
    *outputs.iter().max().unwrap()
}

pub fn part_1(program: Vec<isize>) -> String {
    format!("Part 1: {}", tune_amps_part_1(&program))
}

pub fn part_2(program: Vec<isize>) -> String {
    format!("Part 1: {}", tune_amps_part_2(&program))
}

#[cfg(test)]
mod day_07_tests {
    use super::*;

    #[test]
    fn day_07_sample_part_1_program_1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phases = vec![4, 3, 2, 1, 0];
        let signal_input = 0;
        let mut amp_stack = amp_stack::AmpStack::new();
        amp_stack.init_amps(program, phases);
        let signal_output = amp_stack.process(signal_input);
        assert_eq!(signal_output, 43210);
    }

    #[test]
    fn day_07_sample_part_1_program_2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases = vec![0, 1, 2, 3, 4];
        let signal_input = 0;
        let mut amp_stack = amp_stack::AmpStack::new();
        amp_stack.init_amps(program, phases);
        let signal_output = amp_stack.process(signal_input);
        assert_eq!(signal_output, 54321);
    }
    #[test]
    fn day_07_sample_part_1_program_3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases = vec![1, 0, 4, 3, 2];
        let signal_input = 0;
        let mut amp_stack = amp_stack::AmpStack::new();
        amp_stack.init_amps(program, phases);
        let signal_output = amp_stack.process(signal_input);
        assert_eq!(signal_output, 65210);
    }
    #[test]
    fn day_07_sample_part_2_program_1() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let signal_output = tune_amps_part_2(&program);
        assert_eq!(signal_output, 139629729);
    }
    #[test]
    fn day_07_sample_part_2_program_2() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let signal_output = tune_amps_part_2(&program);
        assert_eq!(signal_output, 18216);
    }
}
