use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> i32 {
    println!("START TEST");
    input.split('\n').fold(0, |total_safe_reports, report| {
        println!("---");
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

enum ProblemDampener {
    Ready,
    Used,
}
fn test_report(mut levels: Vec<i32>) -> bool {
    let mut problem_dampener = ProblemDampener::Ready;
    // Test for all increasing or all decreasing
    loop {
        match (test_direction(&levels), &problem_dampener) {
            // Didn't fail, move onto the next test.
            (None, _) => (),
            // Failed but we have the dampener. Remove the bad level and start the tests over again.
            (Some(fail_point), ProblemDampener::Ready) => {
                levels.remove(fail_point);
                problem_dampener = ProblemDampener::Used;
                continue;
            }
            (Some(_), ProblemDampener::Used) => {
                return false;
            }
        }

        // Test for no more than 3 level changes.
        match (test_distance(&levels), &problem_dampener) {
            // Didn't fail, break out of the loop. We passed all tests.
            (None, _) => break,
            // Failed but we have the dampener. Remove the bad level and start the tests over again.
            (Some(fail_point), ProblemDampener::Ready) => {
                levels.remove(fail_point);
                problem_dampener = ProblemDampener::Used;
                continue;
            }
            (Some(_), ProblemDampener::Used) => {
                return false;
            }
        }
    }

    true
}

#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
enum Direction {
    None,
    Increasing,
    Decreasing,
}
fn test_direction(levels: &[i32]) -> Option<usize> {
    let report_direction = levels
        .windows(2)
        .fold(HashMap::<Direction, i32>::new(), |mut acc, window| {
            let current_level = window[0];
            let next_level = window[1];

            *acc.entry(get_direction(current_level, next_level))
                .or_default() += 1;

            acc
        })
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap();

    for (index, window) in levels.windows(2).enumerate() {
        let current_level = window[0];
        let next_level = window[1];

        let window_is_safe = if current_level == next_level {
            false
        } else {
            report_direction == get_direction(current_level, next_level)
        };

        // Early out from the function if we are not safe.
        if !window_is_safe {
            println!("Failed direction on {} index: {:?}", index, levels);

            if get_direction(current_level, next_level) == report_direction {
                return Some(index + 1);
            } else {
                return Some(index);
            }
        }
    }

    // We must be good at this point
    println!("Passed direction {:?}", levels);
    None
}

fn get_direction(current_level: i32, next_level: i32) -> Direction {
    match current_level.cmp(&next_level) {
        Ordering::Greater => Direction::Decreasing,
        Ordering::Less => Direction::Increasing,
        Ordering::Equal => Direction::None,
    }
}

fn test_distance(levels: &[i32]) -> Option<usize> {
    for (index, window) in levels.windows(2).enumerate() {
        let level1 = window[0];
        let level2 = window[1];

        // Can't have the same levels on a report.
        if level1 == level2 {
            println!("Failed distance (same) on {} index: {:?}", index, levels);
            return Some(index);
        }

        // Make sure the distance is not greater than 3.
        if (level1 - level2).abs() > 3 {
            println!("Failed distance on {} index: {:?}", index, levels);
            return Some(index);
        }
    }

    println!("Passed distance {:?}", levels);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn part_2_tests() {
        part_2_sample();
        part_2_reddit();
    }

    // #[test]
    fn part_2_sample() {
        let result = part_2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn part_2_reddit() {
        let result = part_2(
            "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20",
        );
        assert_eq!(result, 10);
    }
}

// Too Low - 402
// Not the right answer - 404
// Not the right answer - 406
// Not the right answer - 408
// Not the right answer - 410
// Too High - 440
// Too High - 649
