fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> i32 {
    input.split('\n').fold(0, |total_safe_reports, report| {
        let levels: Vec<i32> = report
            .split(' ')
            .map(|level| level.parse::<i32>().expect("Failed to parse level"))
            .collect();

        if test_report(levels) {
            total_safe_reports + 1
        } else {
            total_safe_reports
        }
    })
}

fn test_report(levels: Vec<i32>) -> bool {
    // Test for all increasing or all decreasing
    // Test for no more than 3 level changes.
    if !test_direction(levels.clone()) || !test_distance(levels.clone()) {
        return false;
    }

    true
}

enum Direction {
    None,
    Increasing,
    Decreasing,
}
fn test_direction(levels: Vec<i32>) -> bool {
    let mut current_direction: Direction = Direction::None;
    for window in levels.windows(2) {
        let current_level = window[0];
        let next_level = window[1];

        let window_safe = match (current_level > next_level, &current_direction) {
            // We don't have a direction yet. Set one and move on.
            (decreasing, Direction::None) => {
                current_direction = if decreasing {
                    Direction::Decreasing
                } else {
                    Direction::Increasing
                };

                true
            }
            // We are switching direction. No bueno.
            (true, Direction::Increasing) => false,
            (false, Direction::Decreasing) => false,
            // We aren't in a special case, you may pass.
            _ => true,
        };

        // Early out from the function if we are not safe.
        if !window_safe {
            return false;
        }
    }

    // We must be good at this point
    true
}

fn test_distance(levels: Vec<i32>) -> bool {
    for window in levels.windows(2) {
        let level1 = window[0];
        let level2 = window[1];

        // Can't have the same levels on a report.
        if level1 == level2 {
            return false;
        }

        // Make sure the distance is not greater than 3.
        if (level1 - level2).abs() > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_sample() {
        let result = part_2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 2);
    }
}
