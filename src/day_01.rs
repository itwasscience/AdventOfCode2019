fn calc_module_fuel(mass: i64) -> i64 {
    ((mass as f64 / 3.0).floor() - 2.0) as i64
}

fn calc_all_fuel(mass: i64, total_mass: i64) -> i64 {
    loop {
        match calc_module_fuel(mass) {
            f if f < 0 => return total_mass,
            f if f >= 0 => return calc_all_fuel(f, total_mass + f),
            _ => return 0,
        }
    }
}

pub fn part_1(modules: Vec<i64>) -> String {
    let fuel: i64 = modules.iter().map(|&x| calc_module_fuel(x)).sum();
    format!("Part 1: {}", fuel).to_string()
}

pub fn part_2(modules: Vec<i64>) -> String {
    let total_fuel: i64 = modules.iter().map(|&x| calc_all_fuel(x, 0)).sum();
    format!("Part 2: {}", total_fuel).to_string()
}

#[cfg(test)]
mod day_01_tests {
    use super::*;
    #[test]
    fn part_1() {
        assert_eq!(calc_module_fuel(12), 2);
        assert_eq!(calc_module_fuel(14), 2);
        assert_eq!(calc_module_fuel(1969), 654);
        assert_eq!(calc_module_fuel(100756), 33583);
    }
    #[test]
    fn part_2() {
        assert_eq!(calc_all_fuel(12, 0), 2);
        assert_eq!(calc_all_fuel(1969, 0), 966);
        assert_eq!(calc_all_fuel(100756, 0), 50346);
    }
}
