pub fn run(mut memory: Vec<isize>, ip: usize) -> Vec<isize> {
    loop {
        match memory.iter().nth(ip).unwrap() {
            99 => return memory,
            1 => {
                let a = *memory.iter().nth(ip + 1).unwrap();
                let b = *memory.iter().nth(ip + 2).unwrap();
                let dst = *memory.iter().nth(ip + 3).unwrap();
                if dst >= 0 && a >= 0 && b >= 0 {
                    memory[dst as usize] = memory[a as usize] + memory[b as usize];
                } else {
                    panic!("Memory access at negative index is not allowed!")
                }
                return run(memory, ip + 4);
            }
            2 => {
                let a = *memory.iter().nth(ip + 1).unwrap();
                let b = *memory.iter().nth(ip + 2).unwrap();
                let dst = *memory.iter().nth(ip + 3).unwrap();
                if dst >= 0 && a >= 0 && b >= 0 {
                    memory[dst as usize] = memory[a as usize] * memory[b as usize];
                } else {
                    panic!("Memory access at negative index is not allowed!")
                }
                return run(memory, ip + 4);
            }

            _ => return memory,
        }
    }
}

#[cfg(test)]
mod intcode_tests {
    use super::*;

    #[test]
    fn day_02_opcodes() {
        assert_eq!(*run(vec![1, 0, 0, 0, 99], 0).iter().nth(0).unwrap(), 2);
        assert_eq!(*run(vec![2, 3, 0, 3, 99], 0).iter().nth(3).unwrap(), 6);
        assert_eq!(
            *run(vec![2, 4, 4, 5, 99, 0], 0).iter().nth(5).unwrap(),
            9801
        );
    }
}
