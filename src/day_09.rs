use crate::intcode::intcode::Intcode;

pub fn part_1(program: Vec<isize>) -> String {
    let mut intcode = Intcode::new();
    intcode.load_program(program);
    intcode.set_input(1);
    intcode.run();
    format!("Part 1: {}", intcode.read_output()).to_string()
}

pub fn part_2(program: Vec<isize>) -> String {
    let mut intcode = Intcode::new();
    intcode.load_program(program);
    intcode.set_input(2);
    intcode.run();
    format!("Part 2: {}", intcode.read_output()).to_string()
}
