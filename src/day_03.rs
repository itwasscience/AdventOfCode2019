use std::collections::HashSet;

mod wiring {
    // Why is this not part of the stdlib?
    fn crop_letters(s: &str, pos: usize) -> &str {
        match s.char_indices().skip(pos).next() {
            Some((pos, _)) => &s[pos..],
            None => "",
        }
    }
    #[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    impl Point {
        pub fn manhattan(&self) -> i32 {
            (self.x + self.y).abs()
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Wire {
        pub traces: Vec<Point>,
    }

    impl Wire {
        pub fn new() -> Wire {
            Wire {
                traces: vec![Point { x: 0, y: 0 }],
            }
        }
        pub fn add_trace(&mut self, movement: &String) {
            let direction: char = movement.chars().nth(0).unwrap();
            let distance: i32 = crop_letters(movement, 1).parse().unwrap();
            let last_x = self.traces.last().unwrap().x;
            let last_y = self.traces.last().unwrap().y;

            match direction {
                'U' => self.walk_y_up(last_x, last_y, distance),
                'D' => self.walk_y_down(last_x, last_y, distance),
                'R' => self.walk_x_right(last_x, last_y, distance),
                'L' => self.walk_x_left(last_x, last_y, distance),
                _ => panic!("Inavlid direction received: {}", direction),
            }
        }

        fn walk_x_right(&mut self, last_x: i32, last_y: i32, distance: i32) {
            for x in (last_x + 1)..(last_x + distance + 1) {
                self.traces.push(Point { x: x, y: last_y });
            }
        }

        fn walk_x_left(&mut self, last_x: i32, last_y: i32, distance: i32) {
            for x in ((last_x - distance)..(last_x)).rev() {
                self.traces.push(Point { x: x, y: last_y });
            }
        }

        fn walk_y_up(&mut self, last_x: i32, last_y: i32, distance: i32) {
            for y in (last_y + 1)..(last_y + distance + 1) {
                self.traces.push(Point { x: last_x, y: y });
            }
        }

        fn walk_y_down(&mut self, last_x: i32, last_y: i32, distance: i32) {
            for y in ((last_y - distance)..(last_y)).rev() {
                self.traces.push(Point { x: last_x, y: y });
            }
        }
    }
}

fn build_wire(wire_movements: &String) -> wiring::Wire {
    let mut wire = wiring::Wire::new();
    let movements: Vec<String> = wire_movements.split(",").map(|s| s.to_string()).collect();
    for m in movements {
        wire.add_trace(&m);
    }
    wire
}

fn find_intersections(wire_1: wiring::Wire, wire_2: wiring::Wire) -> Vec<wiring::Point> {
    let w1: HashSet<wiring::Point> = wire_1.traces.into_iter().collect();
    let w2: HashSet<wiring::Point> = wire_2.traces.into_iter().collect();
    let hash_intersections = w1.intersection(&w2);
    let mut intersections = Vec::new();
    for point in hash_intersections {
        if point.x == 0 && point.y == 0 {
            continue;
        }
        intersections.push(*point);
    }
    intersections
}

pub fn part_1(input: Vec<String>) {
    let wire_1_input = input.iter().nth(0).unwrap();
    let wire_2_input = input.iter().nth(1).unwrap();

    let wire_1 = build_wire(wire_1_input);
    let wire_2 = build_wire(wire_2_input);
    let intersections = find_intersections(wire_1, wire_2);
    let mut distances = Vec::new();
    for point in intersections {
        distances.push(point.manhattan());
    }
    distances.sort();
    println!("Day 03, Part 1: {}", distances.iter().nth(0).unwrap());
}

pub fn part_2() -> () {
    println!("Day 03, Part 2: {}", "");
}

#[cfg(test)]
mod day_03_tests {
    use super::*;
    #[test]
    fn part_1() {}
    #[test]
    fn part_2() {}
}

#[cfg(test)]
mod wiring_tests {
    use super::*;
    #[test]
    fn new_wires_have_an_origin_set() {
        let test_wire = wiring::Wire::new();
        let origin = wiring::Point { x: 0, y: 0 };
        assert_eq!(*test_wire.traces.iter().nth(0).unwrap(), origin);
    }
    #[test]
    fn trace_can_add_points() {
        // Movements taken from the example
        let mut test_wire = wiring::Wire::new();
        test_wire.add_trace(&String::from("R8"));
        //println!("{:?}", test_wire.traces);
        test_wire.add_trace(&String::from("U5"));
        //println!("{:?}", test_wire.traces);
        test_wire.add_trace(&String::from("L5"));
        println!("{:?}", test_wire.traces);
        test_wire.add_trace(&String::from("D3"));
        let endpoint: wiring::Point = *test_wire.traces.last().unwrap();
        assert_eq!(wiring::Point { x: 3, y: 2 }, endpoint);
    }
    #[test]
    fn manhattan_distance() {
        assert_eq!(wiring::Point { x: 0, y: 0 }.manhattan(), 0);
        assert_eq!(wiring::Point { x: 3, y: 3 }.manhattan(), 6);
        assert_eq!(wiring::Point { x: -4, y: -4 }.manhattan(), 8);
        assert_eq!(wiring::Point { x: 12, y: 1 }.manhattan(), 13);
        assert_eq!(wiring::Point { x: -3, y: 1 }.manhattan(), 2);
    }
}