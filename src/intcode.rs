pub mod intcode {
    #[derive(Debug, PartialEq, Clone)]
    pub enum IntcodeState {
        Ready,
        Halted,
        WaitingForInput,
    }

    #[derive(Debug)]
    pub struct Intcode {
        memory: Vec<isize>,   // Day 2 - Special memory that can hold negative values
        ip: usize,            // Day 2 - Instruction Pointer
        input: Option<isize>, // Day 5 - External Port
        output: isize,        // Day 5 - External Port
        state: IntcodeState,  // Day 7 - System State Support for dynamic input, deprecates halt
    }

    impl Intcode {
        pub fn new() -> Intcode {
            Intcode {
                memory: Vec::new(),
                ip: 0,
                input: None,
                output: 0,
                state: IntcodeState::Ready,
            }
        }
        pub fn get_state(&self) -> IntcodeState {
            self.state.clone()
        }
        pub fn set_input(&mut self, input: isize) {
            self.input = Some(input.clone());
            // Set state back to ready if intcode is in WaitingForInput state
            self.state = IntcodeState::Ready;
        }
        pub fn read_output(&self) -> isize {
            self.output.clone()
        }
        pub fn load_program(&mut self, program: Vec<isize>) {
            self.memory = program.clone()
        }
        pub fn peek(&mut self, memory_addr: usize) -> isize {
            *self.memory.iter().nth(memory_addr).unwrap()
        }
        pub fn poke(&mut self, memory_addr: usize, value: isize) {
            std::mem::swap(&mut self.memory[memory_addr], &mut value.clone());
        }
        pub fn core_dump(&mut self) {
            println!("{:?}", self);
        }
        pub fn step(&mut self) {
            let value = self.memory.iter().nth(self.ip).unwrap();
            let mut param_1_mode: isize = 0;
            let mut param_2_mode: isize = 0;
            let opcode_ones = value / (10isize.pow(0)) % 10;
            let opcode_tens = value / (10isize.pow(1)) % 10;
            let opcode = (opcode_tens * 10) + opcode_ones;
            // Values over 99 are guarenteed parameter mode opcodes
            if *value > 99 {
                param_1_mode = value / (10isize.pow(2)) % 10;
                param_2_mode = value / (10isize.pow(3)) % 10;
            }
            match opcode {
                1 => self.add(param_1_mode, param_2_mode),
                2 => self.multiply(param_1_mode, param_2_mode),
                3 => self.input(),
                4 => self.output(param_1_mode),
                5 => self.jump_if_true(param_1_mode, param_2_mode),
                6 => self.jump_if_false(param_1_mode, param_2_mode),
                7 => self.less_than(param_1_mode, param_2_mode),
                8 => self.equal(param_1_mode, param_2_mode),
                99 => self.state = IntcodeState::Halted,
                _ => (),
            }
        }
        pub fn run(&mut self) {
            loop {
                if IntcodeState::Ready == self.state {
                    self.step();
                } else {
                    return;
                }
            }
        }
        fn read_mem_loc(&mut self, addr: usize, mode: isize) -> isize {
            if 0 == mode {
                return self.memory[*self.memory.iter().nth(addr).unwrap() as usize];
            } else {
                return *self.memory.iter().nth(addr).unwrap();
            }
        }
        fn add(&mut self, param_1_mode: isize, param_2_mode: isize) {
            let a: isize = self.read_mem_loc(self.ip + 1, param_1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, param_2_mode);
            let dst: isize = self.read_mem_loc(self.ip + 3, 1);
            self.memory[dst as usize] = a + b;
            self.ip += 4;
        }
        fn multiply(&mut self, param_1_mode: isize, param_2_mode: isize) {
            let a: isize = self.read_mem_loc(self.ip + 1, param_1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, param_2_mode);
            let dst: isize = self.read_mem_loc(self.ip + 3, 1);
            self.memory[dst as usize] = a * b;
            self.ip += 4;
        }
        fn input(&mut self) {
            if self.input.is_some() {
                let dst: isize = self.read_mem_loc(self.ip + 1, 1);
                self.memory[dst as usize] = self.input.clone().unwrap();
                self.ip += 2;
                // Flush input buffer
                self.input = None
            } else {
                self.state = IntcodeState::WaitingForInput;
            }
        }
        fn output(&mut self, param_1_mode: isize) {
            let src: isize = self.read_mem_loc(self.ip + 1, param_1_mode);
            self.output = src;
            self.ip += 2;
        }
        fn jump_if_true(&mut self, param_1_mode: isize, param_2_mode: isize) {
            let a: isize = self.read_mem_loc(self.ip + 1, param_1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, param_2_mode);
            if 0 != a {
                self.ip = b as usize;
            } else {
                self.ip += 3;
            }
        }
        fn jump_if_false(&mut self, param_1_mode: isize, param_2_mode: isize) {
            let a: isize = self.read_mem_loc(self.ip + 1, param_1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, param_2_mode);
            if 0 == a {
                self.ip = b as usize;
            } else {
                self.ip += 3;
            }
        }
        fn less_than(&mut self, param_1_mode: isize, param_2_mode: isize) {
            let a: isize = self.read_mem_loc(self.ip + 1, param_1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, param_2_mode);
            let dst: isize = self.read_mem_loc(self.ip + 3, 1);
            if a < b {
                self.memory[dst as usize] = 1;
            } else {
                self.memory[dst as usize] = 0;
            }
            self.ip += 4;
        }
        fn equal(&mut self, param_1_mode: isize, param_2_mode: isize) {
            let a: isize = self.read_mem_loc(self.ip + 1, param_1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, param_2_mode);
            let dst: isize = self.read_mem_loc(self.ip + 3, 1);
            if a == b {
                self.memory[dst as usize] = 1;
            } else {
                self.memory[dst as usize] = 0;
            }
            self.ip += 4;
        }
    }
}

#[cfg(test)]
mod intcode_tests {
    use super::*;

    #[test]
    fn day_02_opcodes_test_addition() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![1, 0, 0, 0, 99]);
        intcode.step();
        intcode.core_dump();
        intcode.step();
        intcode.core_dump();
        assert_eq!(intcode.peek(0), 2);
    }
    #[test]
    fn day_02_opcodes_test_multiplication() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![2, 3, 0, 3, 99]);
        intcode.run();
        assert_eq!(intcode.peek(3), 6);
    }
    #[test]
    fn day_02_opcodes_small_program() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![2, 4, 4, 5, 99, 0]);
        intcode.run();
        assert_eq!(intcode.peek(5), 9801);
    }
    #[test]
    fn test_peek() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![0, 1, 2]);
        assert_eq!(intcode.peek(0), 0);
        assert_eq!(intcode.peek(1), 1);
        assert_eq!(intcode.peek(2), 2);
    }
    #[test]
    fn test_poke() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![0, 0, 0]);
        intcode.poke(0, 0);
        intcode.poke(1, 1);
        intcode.poke(2, 2);
        assert_eq!(intcode.peek(0), 0);
        assert_eq!(intcode.peek(1), 1);
        assert_eq!(intcode.peek(2), 2);
    }
    #[test]
    fn day_05_opcodes_read_in_set_out() {
        let mut intcode = intcode::Intcode::new();
        intcode.set_input(100);
        intcode.load_program(vec![3, 5, 4, 5, 99, 0]);
        intcode.run();
        assert_eq!(intcode.read_output(), 100);
    }
    #[test]
    fn day_05_test_program_1() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![1002, 4, 3, 4, 33]);
        intcode.run();
        assert_eq!(intcode.peek(4), 99);
    }
    #[test]
    fn day_05_test_program_2() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![1101, 100, -1, 4, 0]);
        intcode.run();
        assert_eq!(intcode.peek(4), 99);
    }
    #[test]
    fn day_05_test_program_position_mode_equality_input_0() {
        // Checking for input == 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        intcode.set_input(0);
        intcode.run();
        assert_eq!(intcode.read_output(), 0);
    }
    #[test]
    fn day_05_test_program_position_mode_equality_input_8() {
        // Checking for input == 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        intcode.set_input(8);
        intcode.run();
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_05_test_program_position_mode_less_then_input_10() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        intcode.set_input(10);
        intcode.run();
        assert_eq!(intcode.read_output(), 0);
    }
    #[test]
    fn day_05_test_program_position_mode_less_then_input_3() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_05_test_program_immediate_mode_equality_input_3() {
        // Checking for input == 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(), 0);
    }
    #[test]
    fn day_05_test_program_immediate_mode_equality_input_8() {
        // Checking for input == 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(8);
        intcode.run();
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_05_test_program_immediate_mode_less_than_input_3() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(10);
        intcode.run();
        assert_eq!(intcode.read_output(), 0);
    }
    #[test]
    fn day_05_test_program_immediate_mode_less_than_input_8() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_05_test_program_jump_position_mode_input_0() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        intcode.set_input(0);
        intcode.run();
        assert_eq!(intcode.read_output(), 0);
    }
    #[test]
    fn day_05_test_program_jump_position_mode_input_3() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_05_test_program_jump_immediate_mode_input_0() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        intcode.set_input(0);
        intcode.run();
        assert_eq!(intcode.read_output(), 0);
    }
    #[test]
    fn day_05_test_program_jump_immediate_mode_input_3() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_05_test_program_big_less_than_8() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        intcode.set_input(7);
        intcode.run();
        assert_eq!(intcode.read_output(), 999);
    }
    #[test]
    fn day_05_test_program_big_eq_8() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        intcode.set_input(8);
        intcode.run();
        assert_eq!(intcode.read_output(), 1000);
    }
    #[test]
    fn day_05_test_program_big_greater_than_8() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        intcode.set_input(384);
        intcode.run();
        assert_eq!(intcode.read_output(), 1001);
    }
    #[test]
    fn day_07_test_input_state_pause_with_input_at_start() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_07_test_input_state_pause_with_input_at_pause() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        // Should run until an input opcode is read
        intcode.run();
        assert_eq!(intcode.get_state(), intcode::IntcodeState::WaitingForInput);
        // Provide input and re-run
        intcode.set_input(3);
        intcode.run();
        println!("{:?}", intcode);
        assert_eq!(intcode.read_output(), 1);
    }
    #[test]
    fn day_07_test_input_state_pause_with_input_at_pause_multiple() {
        let mut intcode = intcode::Intcode::new();
        // Program taken from day 7 dual input example
        intcode.load_program(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        // Should run until an input opcode is read
        intcode.run();
        assert_eq!(intcode.get_state(), intcode::IntcodeState::WaitingForInput);
        // Provide input and re-run
        intcode.set_input(0);
        // Should run until an input opcode is read
        intcode.run();
        assert_eq!(intcode.get_state(), intcode::IntcodeState::WaitingForInput);
        // Provide input and re-run
        intcode.set_input(4);
        intcode.run();
        println!("{:?}", intcode);
        assert_eq!(intcode.read_output(), 40);
    }
}
