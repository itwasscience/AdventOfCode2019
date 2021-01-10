pub mod intcode {
    #[derive(Debug, PartialEq, Clone)]
    pub enum IntcodeState {
        Ready,
        Halted,
        WaitingForInput,
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum MemoryMode {
        ImmediateMode,
        PositionMode,
        RelativeMode,
    }
    #[derive(Debug)]
    pub struct Intcode {
        memory: Vec<isize>,   // Day 2 - Special memory that can hold negative values
        ip: usize,            // Day 2 - Instruction Pointer
        input: Option<isize>, // Day 5 - External Port
        output: Vec<isize>,   // Day 5 - External Port, Day 11 - Buffered Output
        state: IntcodeState,  // Day 7 - System State Support for dynamic input, deprecates halt
        relative_base: isize, // Day 9 - Relative base addressing
    }

    impl Intcode {
        pub fn new() -> Intcode {
            Intcode {
                memory: Vec::new(),
                ip: 0,
                input: None,
                output: Vec::new(),
                state: IntcodeState::Ready,
                relative_base: 0,
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
        pub fn read_output(&mut self, position: usize) -> isize {
            *self.output.clone().iter().nth(position).unwrap()
        }
        pub fn flush_output(&mut self) {
            self.output = Vec::new();
        }
        pub fn load_program(&mut self, program: Vec<isize>) {
            self.memory = program.clone();
            // Day 09 - Expand memory greatly
            self.memory.extend(vec![0 as isize; 4000]);
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
            let mut p1_mode = MemoryMode::PositionMode;
            let mut p2_mode = MemoryMode::PositionMode;
            let mut p3_mode = MemoryMode::PositionMode;
            let opcode_ones = value / (10isize.pow(0)) % 10;
            let opcode_tens = value / (10isize.pow(1)) % 10;
            let opcode = (opcode_tens * 10) + opcode_ones;
            // Values over 99 are guarenteed parameter / relative mode opcodes
            if *value > 99 {
                p1_mode = Intcode::decode_mem_mode(value / (10isize.pow(2)) % 10);
                p2_mode = Intcode::decode_mem_mode(value / (10isize.pow(3)) % 10);
                p3_mode = Intcode::decode_mem_mode(value / (10isize.pow(4)) % 10);
            }
            match opcode {
                1 => self.add(p1_mode, p2_mode, p3_mode),
                2 => self.multiply(p1_mode, p2_mode, p3_mode),
                3 => self.input(p1_mode),
                4 => self.output(p1_mode),
                5 => self.jump_if_true(p1_mode, p2_mode),
                6 => self.jump_if_false(p1_mode, p2_mode),
                7 => self.less_than(p1_mode, p2_mode, p3_mode),
                8 => self.equal(p1_mode, p2_mode, p3_mode),
                9 => self.set_relative_base(p1_mode),
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
        pub fn decode_mem_mode(value: isize) -> MemoryMode {
            match value {
                0 => MemoryMode::PositionMode,
                1 => MemoryMode::ImmediateMode,
                2 => MemoryMode::RelativeMode,
                _ => panic!("Unsupported memory mode: {}", value),
            }
        }
        fn read_mem_loc(&mut self, addr: usize, mode: MemoryMode) -> isize {
            let value = *self.memory.iter().nth(addr).unwrap();
            match mode {
                MemoryMode::PositionMode => return self.memory[value as usize],
                MemoryMode::ImmediateMode => return value,
                MemoryMode::RelativeMode => {
                    return self.memory[(value + self.relative_base) as usize]
                }
            }
        }
        fn write_mem_loc(&mut self, addr: usize, value: isize, mode: MemoryMode) {
            match mode {
                // Actually Immediate mode since Position Mode is unsupported for writes
                MemoryMode::PositionMode => self.memory[addr] = value,
                MemoryMode::ImmediateMode => self.memory[addr] = value,
                MemoryMode::RelativeMode => {
                    self.memory[(addr as isize + self.relative_base) as usize] = value
                }
            }
        }
        fn add(&mut self, p1_mode: MemoryMode, p2_mode: MemoryMode, p3_mode: MemoryMode) {
            let a: isize = self.read_mem_loc(self.ip + 1, p1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, p2_mode);
            self.write_mem_loc(self.memory[self.ip + 3] as usize, a + b, p3_mode);
            self.ip += 4;
        }
        fn multiply(&mut self, p1_mode: MemoryMode, p2_mode: MemoryMode, p3_mode: MemoryMode) {
            let a: isize = self.read_mem_loc(self.ip + 1, p1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, p2_mode);
            self.write_mem_loc(self.memory[self.ip + 3] as usize, a * b, p3_mode);
            self.ip += 4;
        }
        fn input(&mut self, p1_mode: MemoryMode) {
            if self.input.is_some() {
                let input = self.input.clone().unwrap();
                self.write_mem_loc(self.memory[self.ip + 1] as usize, input, p1_mode);
                self.ip += 2;
                // Flush input buffer
                self.input = None
            } else {
                self.state = IntcodeState::WaitingForInput;
            }
        }
        fn output(&mut self, p1_mode: MemoryMode) {
            let src: isize = self.read_mem_loc(self.ip + 1, p1_mode);
            self.output.push(src);
            self.ip += 2;
        }
        fn jump_if_true(&mut self, p1_mode: MemoryMode, p2_mode: MemoryMode) {
            let a: isize = self.read_mem_loc(self.ip + 1, p1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, p2_mode);
            if 0 != a {
                self.ip = b as usize;
            } else {
                self.ip += 3;
            }
        }
        fn jump_if_false(&mut self, p1_mode: MemoryMode, p2_mode: MemoryMode) {
            let a: isize = self.read_mem_loc(self.ip + 1, p1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, p2_mode);
            if 0 == a {
                self.ip = b as usize;
            } else {
                self.ip += 3;
            }
        }
        fn less_than(&mut self, p1_mode: MemoryMode, p2_mode: MemoryMode, p3_mode: MemoryMode) {
            let a: isize = self.read_mem_loc(self.ip + 1, p1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, p2_mode);
            if a < b {
                self.write_mem_loc(self.memory[self.ip + 3] as usize, 1, p3_mode);
            } else {
                self.write_mem_loc(self.memory[self.ip + 3] as usize, 0, p3_mode);
            }
            self.ip += 4;
        }
        fn equal(&mut self, p1_mode: MemoryMode, p2_mode: MemoryMode, p3_mode: MemoryMode) {
            let a: isize = self.read_mem_loc(self.ip + 1, p1_mode);
            let b: isize = self.read_mem_loc(self.ip + 2, p2_mode);
            if a == b {
                self.write_mem_loc(self.memory[self.ip + 3] as usize, 1, p3_mode);
            } else {
                self.write_mem_loc(self.memory[self.ip + 3] as usize, 0, p3_mode);
            }
            self.ip += 4;
        }
        fn set_relative_base(&mut self, p1_mode: MemoryMode) {
            self.relative_base = self.relative_base + self.read_mem_loc(self.ip + 1, p1_mode);
            self.ip += 2;
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
        intcode.run();
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
        assert_eq!(intcode.read_output(0), 100);
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
        assert_eq!(intcode.read_output(0), 0);
    }
    #[test]
    fn day_05_test_program_position_mode_equality_input_8() {
        // Checking for input == 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        intcode.set_input(8);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
    }
    #[test]
    fn day_05_test_program_position_mode_less_then_input_10() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        intcode.set_input(10);
        intcode.run();
        assert_eq!(intcode.read_output(0), 0);
    }
    #[test]
    fn day_05_test_program_position_mode_less_then_input_3() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
    }
    #[test]
    fn day_05_test_program_immediate_mode_equality_input_3() {
        // Checking for input == 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(0), 0);
    }
    #[test]
    fn day_05_test_program_immediate_mode_equality_input_8() {
        // Checking for input == 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(8);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
    }
    #[test]
    fn day_05_test_program_immediate_mode_less_than_input_3() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(10);
        intcode.run();
        assert_eq!(intcode.read_output(0), 0);
    }
    #[test]
    fn day_05_test_program_immediate_mode_less_than_input_8() {
        // Checking for input < 8
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
    }
    #[test]
    fn day_05_test_program_jump_position_mode_input_0() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        intcode.set_input(0);
        intcode.run();
        assert_eq!(intcode.read_output(0), 0);
    }
    #[test]
    fn day_05_test_program_jump_position_mode_input_3() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
    }
    #[test]
    fn day_05_test_program_jump_immediate_mode_input_0() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        intcode.set_input(0);
        intcode.run();
        assert_eq!(intcode.read_output(0), 0);
    }
    #[test]
    fn day_05_test_program_jump_immediate_mode_input_3() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
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
        assert_eq!(intcode.read_output(0), 999);
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
        assert_eq!(intcode.read_output(0), 1000);
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
        assert_eq!(intcode.read_output(0), 1001);
    }
    #[test]
    fn day_07_test_input_state_pause_with_input_at_start() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        intcode.set_input(3);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
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
        assert_eq!(intcode.read_output(0), 1);
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
        assert_eq!(intcode.read_output(0), 40);
    }
    #[test]
    fn day_09_test_part_1_program_1() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        intcode.run();
        // It's a quine!
        assert_eq!(intcode.read_output(0), 109);
        assert_eq!(intcode.read_output(1), 1);
        assert_eq!(intcode.read_output(2), 204);
        assert_eq!(intcode.read_output(3), -1);
        assert_eq!(intcode.read_output(4), 1001);
        assert_eq!(intcode.read_output(5), 100);
        assert_eq!(intcode.read_output(6), 1);
        assert_eq!(intcode.read_output(7), 100);
        assert_eq!(intcode.read_output(8), 1008);
        assert_eq!(intcode.read_output(9), 100);
        assert_eq!(intcode.read_output(10), 16);
        assert_eq!(intcode.read_output(11), 101);
        assert_eq!(intcode.read_output(12), 1006);
        assert_eq!(intcode.read_output(13), 101);
        assert_eq!(intcode.read_output(14), 0);
        assert_eq!(intcode.read_output(15), 99);
    }
    #[test]
    fn day_09_test_part_1_program_2() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1219070632396864);
    }
    #[test]
    fn day_09_test_part_1_program_3() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![104, 1125899906842624, 99]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1125899906842624);
    }
    #[test]
    fn day_09_test_part_1_testing_relative_additions() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, 6, 21001, 9, 25, 1, 104, 0, 99, 49]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 74);
    }
    #[test]
    fn day_09_test_part_1_test_suite_1() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, -1, 4, 1, 99]);
        intcode.run();
        assert_eq!(intcode.read_output(0), -1);
    }
    #[test]
    fn day_09_test_part_1_test_suite_2() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, -1, 104, 1, 99]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 1);
    }
    #[test]
    fn day_09_test_part_1_test_suite_3() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, -1, 204, 1, 99]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 109);
    }
    #[test]
    fn day_09_test_part_1_test_suite_4() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, 1, 9, 2, 204, -6, 99]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 204);
    }
    #[test]
    fn day_09_test_part_1_test_suite_5() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, 1, 109, 9, 204, -6, 99]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 204);
    }
    #[test]
    fn day_09_test_part_1_test_suite_6() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, 1, 209, -1, 204, -106, 99]);
        intcode.run();
        assert_eq!(intcode.read_output(0), 204);
    }
    #[test]
    fn day_09_test_part_1_test_suite_7() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, 1, 3, 3, 204, 2, 99]);
        intcode.set_input(42);
        intcode.run();
        assert_eq!(intcode.read_output(0), 42);
    }
    #[test]
    fn day_09_test_part_1_test_suite_8() {
        let mut intcode = intcode::Intcode::new();
        intcode.load_program(vec![109, 1, 203, 2, 204, 2, 99]);
        intcode.set_input(42);
        intcode.run();
        assert_eq!(intcode.read_output(0), 42);
    }
}
