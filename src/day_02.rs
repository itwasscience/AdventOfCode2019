use crate::intcode;

fn fix_crash(mut memory: Vec<usize>) -> Vec<usize> {
    memory[1] = 12;
    memory[2] = 2;
    memory
}

fn attempt_run(mut memory: Vec<usize>, x: usize, y: usize) -> Vec<usize> {
    memory[1] = x;
    memory[2] = y;
    intcode::run(memory, 0)
}

fn brute_force(memory: Vec<usize>) -> (usize, usize) {
    for x in 0..100 {
        for y in 0..100 {
            let copy = memory.clone();
            let value = *attempt_run(copy, x, y).iter().nth(0).unwrap();
            if value == 19_690_720 {
                return (x, y);
            }
        }
    }
    panic!("Match not found for combination of x and y, Aborting!");
}

pub fn part_1(mut memory: Vec<usize>) -> () {
    memory = fix_crash(memory);
    let result = *intcode::run(memory, 0).iter().nth(0).unwrap();
    println!("Part 1: {}", result);
}

pub fn part_2(memory: Vec<usize>) -> () {
    let (x, y) = brute_force(memory);
    println!("Part 2: {}", 100 * x + y)
}

#[cfg(test)]
mod day_02_tests {
    #[test]
    fn part_1() {
        // Moved to intcode module
    }
    #[test]
    fn part_2() {}
}
