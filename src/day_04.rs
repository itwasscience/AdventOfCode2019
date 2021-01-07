use std::collections::HashMap;

pub fn calc_combos(start: u32, end: u32) -> (u32, u32) {
    let mut found_rule_1: u32 = 0;
    let mut found_rule_2: u32 = 0;
    for combo in start..end {
        let (d6, d5, d4, d3, d2, d1) = combo_to_digits(combo);
        if is_valid_ruleset_1(d6, d5, d4, d3, d2, d1) {
            found_rule_1 += 1;
            if is_valid_ruleset_2(d6, d5, d4, d3, d2, d1) {
                found_rule_2 += 1;
            }
        }
    }
    (found_rule_1, found_rule_2)
}

fn combo_to_digits(combo: u32) -> (u32, u32, u32, u32, u32, u32) {
    (
        combo / (10u32.pow(5)) % 10,
        combo / (10u32.pow(4)) % 10,
        combo / (10u32.pow(3)) % 10,
        combo / (10u32.pow(2)) % 10,
        combo / (10u32.pow(1)) % 10,
        combo / (10u32.pow(0)) % 10,
    )
}

pub fn is_valid_ruleset_2(d6: u32, d5: u32, d4: u32, d3: u32, d2: u32, d1: u32) -> bool {
    let mut m: HashMap<u32, usize> = HashMap::new();
    for x in vec![d6, d5, d4, d3, d2, d1] {
        *m.entry(x).or_default() += 1;
    }
    return m.values().any(|&val| val == 2);
}

pub fn is_valid_ruleset_1(d6: u32, d5: u32, d4: u32, d3: u32, d2: u32, d1: u32) -> bool {
    // Rule 1
    if (d1 < d2) || (d2 < d3) || (d3 < d4) || (d4 < d5) || (d5 < d6) {
        return false;
    }
    // Rule 2
    if (d1 == d2) || (d2 == d3) || (d3 == d4) || (d4 == d5) || (d5 == d6) {
        return true;
    }
    return false;
}

pub fn part_1() -> String {
    let (found, _) = calc_combos(124075, 580769);
    format!("Part 1: {}", found).to_string()
}

pub fn part_2() -> String {
    let (_, found) = calc_combos(124075, 580769);
    format!("Part 2: {}", found).to_string()
}

#[cfg(test)]
mod day_04_tests {
    use super::*;
    #[test]
    fn test_valid_ruleset_1() {
        assert_eq!(is_valid_ruleset_1(1, 1, 1, 1, 1, 1), true);
        assert_eq!(is_valid_ruleset_1(1, 2, 3, 4, 4, 5), true);
        assert_eq!(is_valid_ruleset_1(2, 2, 3, 4, 5, 0), false);
        assert_eq!(is_valid_ruleset_1(1, 2, 3, 7, 8, 9), false);
        assert_eq!(is_valid_ruleset_1(1, 2, 3, 4, 4, 3), false);
    }
    #[test]
    fn test_valid_ruleset_2() {
        assert_eq!(is_valid_ruleset_2(1, 1, 2, 2, 3, 3), true);
        assert_eq!(is_valid_ruleset_2(1, 2, 3, 4, 4, 4), false);
        assert_eq!(is_valid_ruleset_2(1, 1, 1, 1, 2, 2), true);
    }
}
