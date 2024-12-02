fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let result = part1(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 11);
    }
}
