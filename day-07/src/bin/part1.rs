use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Operator {
    Multiply(i32, i32),
    Add(i32, i32),
}

impl Operator {
    fn eval(self) -> i32 {
        match self {
            Operator::Multiply(a, b) => a * b,
            Operator::Add(a, b) => a + b,
        }
    }
}

#[derive(Debug)]
struct Equation {
    test_value: i32,
    numbers: Vec<i32>,
}

fn recursive_eval(acc: i32, equation: &mut Equation) -> bool {
    let a = equation.numbers.pop();

    if a.is_none() {
        return false;
    }

    let a = a.unwrap();

    dbg!(&acc, &a);

    if Operator::Multiply(acc, a).eval() == equation.test_value {
        return true;
    }
    if Operator::Add(acc, a).eval() == equation.test_value {
        return true;
    }

    recursive_eval(acc, equation)
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    let (input, parsed_equations) = separated_list0(
        newline,
        separated_pair(
            complete::i32,
            tag(": "),
            separated_list0(space1, complete::i32),
        ),
    )(input)?;

    let equations = parsed_equations
        .into_iter()
        .map(|(test_value, numbers)| Equation {
            test_value,
            numbers,
        })
        .collect::<Vec<Equation>>();

    Ok((input, equations))
}

fn part_1(input: &str) -> i32 {
    let (_, equations) = parse(input).expect("Failed to parse input");

    equations.into_iter().fold(0, |acc, mut equation| {
        let test_value = equation.test_value;
        let first_value = equation.numbers.pop().unwrap();
        if recursive_eval(first_value, &mut equation) {
            println!("Equation good: {:?}", &equation);
            acc + test_value
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let result = part_1(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, 3749);
    }
}
