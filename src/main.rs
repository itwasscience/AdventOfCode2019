use colored::*;
mod day_01;
mod day_02;
mod day_03;
mod helpers;
mod intcode;

fn print_divider_green(day: &str) {
    println!("{}", "----------------".bright_green());
    println!("{} {}", "Day ".bright_green(), day.bright_green());
    println!("{}", "----------------".bright_green());
}

fn print_divider_red(day: &str) {
    println!("{}", "----------------".bright_red());
    println!("{} {}", "Day ".bright_red(), day.bright_red());
    println!("{}", "----------------".bright_red());
}

fn main() {
    println!("Advent of Code 2019");
    // Day 1
    print_divider_green("01");
    day_01::part_1(helpers::read_file_ints("./inputs/day_01.txt").unwrap());
    day_01::part_2(helpers::read_file_ints("./inputs/day_01.txt").unwrap());
    // Day 2
    print_divider_red("02");
    day_02::part_1(helpers::read_file_delim_ints("./inputs/day_02.txt", ",").unwrap());
    day_02::part_2(helpers::read_file_delim_ints("./inputs/day_02.txt", ",").unwrap());
    // Day 3
    print_divider_green("03");
    day_03::part_1(helpers::read_file("./inputs/day_03.txt").unwrap());
    day_03::part_2(helpers::read_file("./inputs/day_03.txt").unwrap());
}
