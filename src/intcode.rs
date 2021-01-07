pub mod intcode {
    #[derive(Debug)]
    pub struct Intcode {
        memory: Vec<isize>, // Special memory that can hold negative values
        ip: usize,          // Instruction Pointer
        input: isize,       // External Port
        output: isize,      // External Port
        halt: bool,         // Halting Flag - Opcode 99
    }

    impl Intcode {
        pub fn new() -> Intcode {
            Intcode {
                memory: Vec::new(),
                ip: 0,
                input: 0,
                output: 0,
                halt: false,
            }
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
        pub fn step(&mut self) {
            // println!("{:?}", self);
            match self.memory.iter().nth(self.ip).unwrap() {
                99 => {
                    self.halt = true;
                }
                1 => {
                    let a = *self.memory.iter().nth(self.ip + 1).unwrap();
                    let b = *self.memory.iter().nth(self.ip + 2).unwrap();
                    let dst = *self.memory.iter().nth(self.ip + 3).unwrap();
                    if dst >= 0 && a >= 0 && b >= 0 {
                        self.memory[dst as usize] =
                            self.memory[a as usize] + self.memory[b as usize];
                    } else {
                        panic!("self.memory access at negative index is not allowed!")
                    }
                    self.ip += 4;
                }
                2 => {
                    let a = *self.memory.iter().nth(self.ip + 1).unwrap();
                    let b = *self.memory.iter().nth(self.ip + 2).unwrap();
                    let dst = *self.memory.iter().nth(self.ip + 3).unwrap();
                    if dst >= 0 && a >= 0 && b >= 0 {
                        self.memory[dst as usize] =
                            self.memory[a as usize] * self.memory[b as usize];
                    } else {
                        panic!("self.memory access at negative index is not allowed!")
                    }
                    self.ip += 4;
                }
                _ => (),
            }
        }
        pub fn run(&mut self) {
            loop {
                if self.halt {
                    return;
                } else {
                    self.step();
                }
            }
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
}
