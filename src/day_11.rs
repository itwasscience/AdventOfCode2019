use itertools::Itertools;
pub mod robot {
    use crate::intcode::intcode::Intcode;
    use crate::intcode::intcode::IntcodeState;

    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Panel {
        pub x: isize,
        pub y: isize,
        pub color: isize,
    }
    #[derive(Debug)]
    pub struct Robot {
        brain: Intcode,
        path: Vec<Panel>,
        direction: Direction,
    }

    impl Robot {
        pub fn new(starting_color: isize) -> Robot {
            let mut path = Vec::new();
            path.push(Panel {
                x: 0,
                y: 0,
                color: starting_color,
            });
            Robot {
                brain: Intcode::new(),
                path: path,
                direction: Direction::Up,
            }
        }
        pub fn get_path(&self) -> Vec<Panel> {
            let mut path = Vec::new();
            for p in self.path.clone() {
                path.push(p.clone())
            }
            path
        }
        pub fn load_program(&mut self, program: Vec<isize>) {
            self.brain.load_program(program);
        }
        pub fn run(&mut self) {
            loop {
                if IntcodeState::Halted == self.brain.get_state() {
                    return;
                } else {
                    self.step();
                }
            }
        }
        pub fn step(&mut self) {
            // Paths are always added to the front of the vec so we can easily
            // find the last-seen tile since find() returns the first hit down
            // the vec. If we were to push the logic would be wrong when scanning.

            // Take the tile we're currently on as input
            let current_panel = &self.path[0].clone();
            self.brain.set_input(current_panel.color);
            self.brain.run();
            let paint_color = self.brain.read_output(0);
            let direction = self.brain.read_output(1);
            self.brain.flush_output();
            // Paint the current tile _before_ moving
            self.path.first_mut().unwrap().color = paint_color;
            // Now we're allowed to move
            let (delta_x, delta_y) = self.roll(direction);
            // Have we been here before? If so save the color for the new panel, else black
            let seen_tile_color = match self
                .path
                .iter()
                .find(|&p| p.x == current_panel.x + delta_x && p.y == current_panel.y + delta_y)
            {
                Some(seen_tile) => seen_tile.color,
                None => 0,
            };
            // Finally insert it as the new FIRST entry to the vector, see note at top
            let new_panel = Panel {
                x: current_panel.x + delta_x,
                y: current_panel.y + delta_y,
                color: seen_tile_color,
            };
            self.path.insert(0, new_panel);
        }

        fn roll(&mut self, direction: isize) -> (isize, isize) {
            let mut delta_x: isize = 0;
            let mut delta_y: isize = 0;
            if 1 == direction {
                // Clockwise Rotations
                match &self.direction {
                    Direction::Up => {
                        self.direction = Direction::Right;
                        delta_x += 1;
                    }
                    Direction::Right => {
                        self.direction = Direction::Down;
                        delta_y -= 1;
                    }
                    Direction::Down => {
                        self.direction = Direction::Left;
                        delta_x -= 1;
                    }
                    Direction::Left => {
                        self.direction = Direction::Up;
                        delta_y += 1;
                    }
                }
            } else {
                // Counter-Clockwise Rotations
                match &self.direction {
                    Direction::Up => {
                        self.direction = Direction::Left;
                        delta_x -= 1;
                    }
                    Direction::Left => {
                        self.direction = Direction::Down;
                        delta_y -= 1;
                    }
                    Direction::Down => {
                        self.direction = Direction::Right;
                        delta_x += 1;
                    }
                    Direction::Right => {
                        self.direction = Direction::Up;
                        delta_y += 1;
                    }
                }
            }
            (delta_x, delta_y)
        }
    }
}

pub fn part_1(program: Vec<isize>) -> String {
    let mut robot = robot::Robot::new(0);
    robot.load_program(program);
    robot.run();

    let painted_panels: Vec<robot::Panel> = robot
        .get_path()
        .into_iter()
        .unique_by(|p| (p.x, p.y))
        .collect();
    format!("Part 1: {}", painted_panels.len())
}

pub fn part_2(program: Vec<isize>) -> String {
    let mut robot = robot::Robot::new(1);
    robot.load_program(program);
    robot.run();

    let painted_panels: Vec<robot::Panel> = robot.get_path();
    // Coords may be negative, calculate an offset suitable for drawing
    let x_vals: Vec<isize> = painted_panels.iter().map(|p| p.x).collect();
    let x_min = *x_vals.iter().min().unwrap();
    let x_max = *x_vals.iter().max().unwrap();
    let y_vals: Vec<isize> = painted_panels.iter().map(|p| p.y).collect();
    let y_min = *y_vals.iter().min().unwrap();
    let y_max = *y_vals.iter().max().unwrap();

    let mut output = "\n".to_owned();
    for y in (y_min..y_max + 1).rev() {
        for x in x_min..x_max + 1 {
            let color = match painted_panels.iter().find(|&p| p.x == x && p.y == y) {
                Some(panel) => panel.color,
                None => 0,
            };
            if 0 == color {
                output.push_str(&" ");
            } else {
                output.push_str(&"#");
            }
        }
        output.push_str(&"\n");
    }
    format!("Part 2: {}", output)
}
