use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::{map, value},
    multi::{many0, many1, many_till},
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};
// This part was a walk through from Chris Biscardi (https://www.youtube.com/watch?v=Ja7nETLnsXQ)
fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn part_2(input: &str) -> u32 {
    let (_, result) = parse(input).unwrap();

    result
        .iter()
        .fold(
            (Instruction::Do, 0),
            |(last_instruction, acc), instruction| match *instruction {
                Instruction::Mul(x, y) => {
                    if last_instruction == Instruction::Do {
                        return (last_instruction, acc + (x * y));
                    }

                    (last_instruction, acc)
                }
                Instruction::Do => (Instruction::Do, acc),
                Instruction::Dont => (Instruction::Dont, acc),
            },
        )
        .1
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_, instruction)| instruction))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_sample() {
        let result =
            part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }
}
