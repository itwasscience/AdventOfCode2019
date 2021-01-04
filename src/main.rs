mod day_01;
mod helpers;

fn main() {
    println!("Advent of Code 2019");
    day_01::part_1(helpers::read_file_ints("./inputs/day_01.txt").unwrap());
}
