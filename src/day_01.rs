fn calc_module_fuel(mass: i64) -> i64 {
    ((mass as f64 / 3.0).floor() - 2.0) as i64
}

pub fn part_1() {}

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
}
