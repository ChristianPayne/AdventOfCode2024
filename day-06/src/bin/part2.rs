use std::{collections::HashSet, ops::Add};

#[derive(Debug)]
struct Board {
    dimensions: (u32, u32),
    guard: Guard,
    obstacles: HashSet<Position>,
}

#[derive(Debug)]
struct Guard {
    position: Position,
    starting_position: Position,
    direction: Direction,
    positions_visited: HashSet<(Position, Direction)>,
    max_moves_allowed: i32,
    total_moves: i32,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position(i32, i32);

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

        self.obstacles.contains(&position_to_check.0)
    }

    fn move_guard(&mut self) {
        if !self.check_for_obstacle() {
            let next_position = self.guard.get_next_position();
            self.guard.position = next_position.0;
            self.guard.positions_visited.insert(next_position);
            self.guard.total_moves += 1;
            // println!("Guard moved to: {:?}", next_position);
        } else {
            self.guard.change_direction();
            let next_position = self.guard.get_next_position();
            self.guard.positions_visited.insert(next_position);
            // println!("Guard changed to direction: {:?}", self.guard.direction);
        }
    }
    fn place_obstacle(&mut self, position: Position) -> Option<Position> {
        if self.obstacles.contains(&position) {
            return None;
        }

        self.obstacles.insert(position);

        Some(position)
    }

    fn remove_obstacle(&mut self, position: Position) {
        self.obstacles.remove(&position);
    }

    fn reset_guard_to_starting_position(&mut self) {
        self.guard.position = self.guard.starting_position;
        self.guard.direction = Direction::Up;
    }

    fn check_if_guard_loops(&mut self) -> bool {
        while self.guard_on_board() {
            // We might be caught in an unsolvable case (running back and forth and not a proper loop).
            if self.guard.total_moves > self.guard.max_moves_allowed {
                break;
            }

            self.move_guard();

            let guard_vector = (self.guard.position, self.guard.direction);
            let next_vector = self.guard.get_next_position();

            if self.guard.positions_visited.contains(&guard_vector)
                && self.guard.positions_visited.contains(&next_vector)
            {
                return true;
            }
        }

        false
    }

    fn clear_positions_visited(&mut self) {
        self.guard.positions_visited.clear();
        self.guard.total_moves = 0;
    }
}

impl Guard {
    fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn get_next_position(&self) -> (Position, Direction) {
        match self.direction {
            Direction::Up => (self.position + Position(0, -1), self.direction),
            Direction::Down => (self.position + Position(0, 1), self.direction),
            Direction::Left => (self.position + Position(-1, 0), self.direction),
            Direction::Right => (self.position + Position(1, 0), self.direction),
        }
    }
}

fn parse(input: &str) -> Board {
    let mut board = Board {
        dimensions: (0, 0),
        guard: Guard {
            direction: Direction::Up,
            position: Position(0, 0),
            starting_position: Position(0, 0),
            positions_visited: HashSet::new(),
            max_moves_allowed: 6000,
            total_moves: 0,
        },
        obstacles: HashSet::new(),
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
                    board.guard.starting_position = pos;
                    board.guard.positions_visited.insert((pos, Direction::Up));
                }
                '#' => {
                    board.obstacles.insert(Position(x as i32, y as i32));
                }
                _ => (),
            }
        }
    }

    board
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> i32 {
    let mut board = parse(input);

    // Do a full run. Gather all the places where we should test a box.
    board.check_if_guard_loops();

    // Gather all the places we have visited (except the first place) to test putting obstacles.
    let positions_to_test_obstacles = board
        .guard
        .positions_visited
        .iter()
        .filter_map(|vector| {
            // Skip the starting and ending guard positions.
            if vector.0 == board.guard.starting_position {
                return None;
            }

            // Reduce the vector to just the positions of where the guard has been (stripping the direction).
            Some(vector.0)
        })
        // Collecting this into a hash set because we may have duplicates where the guard was as the same position, just in another direction.
        .collect::<HashSet<Position>>();

    positions_to_test_obstacles
        .into_iter()
        .fold(0, |acc, position| {
            // println!("Starting check on {:?}", position);
            // Make sure we aren't checking the visited positions of past tests.
            board.clear_positions_visited();

            // Reset the guard to start the new test.
            board.reset_guard_to_starting_position();

            // Set up the new obstacle.
            board.place_obstacle(position);

            if board.check_if_guard_loops() {
                // Clean up the obstacle we are testing.
                board.remove_obstacle(position);

                // println!("Obstacle at {:?} loops.", position);

                // Increment the ones that looped properly.
                return acc + 1;
            } else {
                // println!("Obstacle at {:?} does not loop.", position);
            }

            // Clean up the obstacle we are testing.
            board.remove_obstacle(position);
            // This iteration did not loop properly. Don't include it in the total.
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let result = part_2(
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
        assert_eq!(result, 6);
    }
}

// 1714 Too high
// 266 Too low
// 1567 Too low
// 1566 Too low
