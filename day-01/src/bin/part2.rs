fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> i32 {
    let mut lists: (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    for row in input.split('\n') {
        for (i, num) in row.split("   ").enumerate() {
            match i {
                0 => lists.0.push(num.parse::<i32>().unwrap()),
                1 => lists.1.push(num.parse::<i32>().unwrap()),
                _ => panic!("Failed parsing!"),
            }
        }
    }

    lists.0.sort();
    lists.1.sort();

    let mut acc = 0;
    for val in lists.0 {
        let times_in_other_list = lists.1.iter().filter(|x| **x == val).count();

        acc += val * times_in_other_list as i32;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_sample() {
        let result = part_2(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 31);
    }
}
