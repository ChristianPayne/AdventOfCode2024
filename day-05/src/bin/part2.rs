use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct SortRule(i32, i32);

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> i32 {
    let (_, (rules, _, updates)) = tuple((
        separated_list0(tag("\n"), parse_sort_rule),
        tag("\n\n"),
        separated_list0(tag("\n"), parse_updates),
    ))(input)
    .expect("Failed to parse input.");

    // Make sure each update is in order and keep a sum of the middle pages.
    updates.into_iter().fold(0, |acc, mut update| {
        // Find valid rules
        let rules_for_update: Vec<SortRule> = rules
            .clone()
            .into_iter()
            .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
            .collect();

        // Make sure we only work on the updates that are incorrect.
        if !check_update_passed(&update, &rules_for_update) {
            // Sort the update based on the rules.
            update = sort_update(update, &rules_for_update);

            // Get the middle page.
            let middle_page_index = update.len() / 2;
            if let Some(page) = update.get(middle_page_index) {
                // println!("Middle page number: {}", page);
                acc + page
            } else {
                println!("Failed to get page: {}", middle_page_index);
                acc
            }
        } else {
            acc
        }
    })
}

fn sort_update(mut update: Vec<i32>, rules: &[SortRule]) -> Vec<i32> {
    println!("Starting sort on {:?}", update);

    loop {
        let all_rules_passed = rules.iter().all(|rule| {
            if !check_rule_passed(&update, rule) {
                let rule_0_index = update
                    .iter()
                    .position(|x| *x == rule.0)
                    .expect("Failed to find index of rule 0");
                let rule_1_index = update
                    .iter()
                    .position(|x| *x == rule.1)
                    .expect("Failed to find index of rule 0");
                update.swap(rule_0_index, rule_1_index);
                return false;
            }

            true
        });

        if all_rules_passed {
            break;
        }
    }

    update
}

fn check_update_passed(update: &Vec<i32>, rules: &Vec<SortRule>) -> bool {
    // Check all rules to make sure the update is good for each.
    for rule in rules {
        if !check_rule_passed(update, rule) {
            return false;
        }
    }

    // println!("Update passed: {:?}", update);
    true
}

fn check_rule_passed(update: &Vec<i32>, rule: &SortRule) -> bool {
    let index = update
        .iter()
        .position(|x| *x == rule.0)
        .expect("Failed to find index of rule page 0");

    let mut left_split = update.clone();
    let right_split = left_split.split_off(index);

    // This would mean we failed a rule and thus the entire update.
    if !right_split.contains(&rule.1) {
        return false;
    }

    true
}

fn parse_sort_rule(input: &str) -> IResult<&str, SortRule> {
    let (input, (x, y)) = separated_pair(complete::i32, tag("|"), complete::i32)(input)?;
    Ok((input, SortRule(x, y)))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(","), complete::i32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_sample() {
        let result = part_2(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 123);
    }
}
