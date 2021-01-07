use colored::*;
use std::time::Instant;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod helpers;
mod intcode;

fn print_divider_green(day: String, p1_result: String, p2_result: String) {
    println!("{}", "-----------------------".bright_green());
    println!("{} {}", "Day ".bright_green(), day.bright_green());
    println!("{}", "-----------------------".bright_green());
    println!("{}\n{}", p1_result, p2_result);
}

fn print_divider_red(day: String, p1_result: String, p2_result: String) {
    println!("{}", "-----------------------".bright_red());
    println!("{} {}", "Day ".bright_red(), day.bright_red());
    println!("{}", "-----------------------".bright_red());
    println!("{}\n{}", p1_result, p2_result);
}

fn main() {
    println!("{}", "Advent of Code 2019".bright_white());
    // Day 1
    let start = Instant::now();
    let p1 = day_01::part_1(helpers::read_file_ints("./inputs/day_01.txt").unwrap());
    let p2 = day_01::part_2(helpers::read_file_ints("./inputs/day_01.txt").unwrap());
    print_divider_green(format!("01 - {:?}", start.elapsed()), p1, p2);

    // Day 2
    let start = Instant::now();
    let p1 = day_02::part_1(helpers::read_file_delim_ints("./inputs/day_02.txt", ",").unwrap());
    let p2 = day_02::part_2(helpers::read_file_delim_ints("./inputs/day_02.txt", ",").unwrap());
    print_divider_red(format!("02 - {:?}", start.elapsed()), p1, p2);

    // Day 3
    let start = Instant::now();
    let p1 = day_03::part_1(helpers::read_file("./inputs/day_03.txt").unwrap());
    let p2 = day_03::part_2(helpers::read_file("./inputs/day_03.txt").unwrap());
    print_divider_green(format!("03 - {:?}", start.elapsed()), p1, p2);

    // Day 4
    let start = Instant::now();
    let p1 = day_04::part_1();
    let p2 = day_04::part_2();
    print_divider_red(format!("04 - {:?}", start.elapsed()), p1, p2);
}
