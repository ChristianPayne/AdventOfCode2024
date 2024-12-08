use std::{collections::HashMap, ops::Add};

#[derive(Debug)]
struct Board {
    dimensions: (u32, u32),
    guard: Guard,
    obstacles: HashMap<Position, bool>,
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
    positions_visited: HashMap<Position, bool>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position(i32, i32);

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Board {
    fn guard_on_board(&self) -> bool {
        // Check X dimension.
        if self.guard.position.0 > (self.dimensions.0 - 1) as i32 || self.guard.position.0 < 0 {
            return false;
        }
        // Check Y dimension.
        if self.guard.position.1 > (self.dimensions.1 - 1) as i32 || self.guard.position.1 < 0 {
            return false;
        }

        true
    }

    fn check_for_obstacle(&self) -> bool {
        let position_to_check = self.guard.get_next_position();

        self.obstacles.contains_key(&position_to_check)
    }

    fn move_guard(mut self) -> Self {
        if !self.check_for_obstacle() {
            self.guard
                .positions_visited
                .insert(self.guard.position, true);
            let next_position = self.guard.get_next_position();
            self.guard.position = next_position;
            println!("Guard moved to: {:?}", self.guard.position);
        } else {
            self.guard.change_direction();
            println!("Guard changed to direction: {:?}", self.guard.direction);
        }

        self
    }
}

impl Guard {
    fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_next_position(&self) -> Position {
        match self.direction {
            Direction::Up => self.position + Position(0, -1),
            Direction::Down => self.position + Position(0, 1),
            Direction::Left => self.position + Position(-1, 0),
            Direction::Right => self.position + Position(1, 0),
        }
    }
}

fn parse(input: &str) -> Board {
    let mut board = Board {
        dimensions: (0, 0),
        guard: Guard {
            direction: Direction::Up,
            position: Position(0, 0),
            positions_visited: HashMap::new(),
        },
        obstacles: HashMap::new(),
    };

    let (x_dimension, y_dimension) = (
        input.split('\n').count(),
        input
            .split('\n')
            .last()
            .map(|last_line| last_line.chars().count())
            .expect("Failed to get last item"),
    );
    board.dimensions = (x_dimension as u32, y_dimension as u32);

    for (y, line) in input.split('\n').enumerate() {
        // Something
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    let pos = Position(x as i32, y as i32);
                    board.guard.position = pos;
                    board.guard.positions_visited.insert(pos, true);
                }
                '#' => {
                    board.obstacles.insert(Position(x as i32, y as i32), true);
                }
                _ => (),
            }
        }
    }

    // dbg!(&board);

    board
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> i32 {
    let mut board = parse(input);

    while board.guard_on_board() {
        board = board.move_guard();
    }

    board.guard.positions_visited.keys().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let result = part_1(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, 41);
    }
}
