use nom::{bytes::complete::tag, Err, IResult, Parser};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> i32 {
    todo!()
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("mul")(input)?;
    let (input, x) = nom::character::complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = nom::character::complete::i32(input)?;
    Ok((input, (x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let result = part_1("");
        assert_eq!(result, 2);
    }
}
