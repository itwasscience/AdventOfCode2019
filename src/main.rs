mod day_01;
mod day_02;
mod helpers;
mod intcode;

fn main() {
    println!("Advent of Code 2019");
    // Day 1
    day_01::part_1(helpers::read_file_ints("./inputs/day_01.txt").unwrap());
    day_01::part_2(helpers::read_file_ints("./inputs/day_01.txt").unwrap());
    // Day 2
    day_02::part_1(helpers::read_file_delim_ints("./inputs/day_02.txt", ",").unwrap());
    day_02::part_2(helpers::read_file_delim_ints("./inputs/day_02.txt", ",").unwrap());
}
