use crate::intcode::intcode::Intcode;

pub fn part_1(program: Vec<isize>) -> String {
    let mut intcode = Intcode::new();
    intcode.load_program(program);
    // Fix Crash
    intcode.poke(1, 12);
    intcode.poke(2, 2);
    intcode.run();
    format!("Part 1: {}", intcode.peek(0)).to_string()
}

pub fn part_2(program: Vec<isize>) -> String {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    for i in 0..100 {
        for j in 0..100 {
            let clone = program.clone();
            let mut intcode = Intcode::new();
            intcode.load_program(clone);
            intcode.poke(1, i); // Noun
            intcode.poke(2, j); // Verb
            intcode.run();
            if intcode.peek(0) == 19_690_720 {
                x = i as i16;
                y = j as i16;
            }
        }
    }
    format!("Part 2: {}", 100 * x + y).to_string()
}
