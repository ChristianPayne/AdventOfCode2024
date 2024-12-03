use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::map,
    multi::many0,
    sequence::{separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> u32 {
    let (_, result) = parse_all_mul(input).unwrap();

    result.iter().fold(0, |acc, (x, y)| acc + (x * y))
}

fn parse_all_mul(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many0(parse_mul_or_skip)(input).map(|(next_input, results)| {
        // Filter out `None` results
        (next_input, results.into_iter().flatten().collect())
    })
}

fn parse_mul_or_skip(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    alt((
        map(parse_mul, Some),           // Parse valid "mul"
        map(consume_invalid, |_| None), // Consume one invalid character
    ))(input)
}

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (_, (x, y), _)) = tuple((tag("mul("), parse_numbers, tag(")")))(input)?;

    Ok((input, (x, y)))
}

fn consume_invalid(input: &str) -> IResult<&str, ()> {
    map(anychar, |_| ())(input)
}

fn parse_numbers(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, tag(","), complete::u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_test() {
        let (_, result) = parse_numbers("2,4").unwrap();
        assert_eq!(result, (2, 4));
    }
    #[test]
    fn parse_mul_test() {
        let (_, result) = parse_mul("mul(2,4)").unwrap();
        assert_eq!(result, (2, 4));
    }
    #[test]
    fn part_1_sample() {
        let result =
            part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
}
