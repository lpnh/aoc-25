use anyhow::{Context, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_06.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<i64> {
    let mut worksheet = input.lines();

    let operations: Vec<char> = worksheet
        .next_back()
        .context("failed to get last line")?
        .replace(" ", "")
        .chars()
        .collect();

    // array of worksheet lines (horizontal)
    let list: Vec<Vec<&str>> = worksheet.map(|l| l.split_whitespace().collect()).collect();

    // array of problems (vertical)
    let problem = vec![0; list.len()]; // one number per worksheet line
    let mut problems = vec![problem; list[0].len()]; // one problem per worksheet column

    for i in 0..problems.len() {
        for j in 0..problems[0].len() {
            problems[i][j] = list[j][i]
                .parse::<i64>()
                .context("failed to parse number")?;
        }
    }

    let grand_total = operations
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            '+' => problems[i].iter().sum::<i64>(),
            '*' => problems[i].iter().product(),
            _ => unreachable!(),
        })
        .sum::<i64>();

    Ok(grand_total)
}

#[cfg(feature = "part_2")]
fn solution_part_2(input: &str) -> Result<u64> {
    let mut worksheet = input.lines();

    let rev_op: Vec<char> = worksheet
        .next_back()
        .context("failed to get last line")?
        .chars()
        .rev()
        .collect();

    // array of worksheet reverted lines (horizontal)
    let rev_list: Vec<Vec<Vec<char>>> = worksheet
        .map(|line| {
            let line_rev: Vec<char> = line.chars().rev().collect();
            let mut numbers = Vec::<Vec<char>>::new();
            let mut split_at = 0;
            for op_idx in 0..rev_op.len() - 1 {
                if !rev_op[op_idx].is_whitespace() {
                    numbers.push(line_rev[split_at..=op_idx].into());
                    split_at = op_idx + 2;
                }
            }
            numbers.push(line_rev[split_at..].into());
            numbers
        })
        .collect();

    // array of problems (vertical)
    let mut problems = Vec::<Vec<u64>>::new();
    for i in 0..rev_list[0].len() {
        let mut problem = Vec::<u64>::new();
        for j in 0..rev_list[0][i].len() {
            let mut number_as_str = String::new();
            for line in &rev_list {
                let digit_as_char = line[i][j];
                if !digit_as_char.is_whitespace() {
                    number_as_str.push(digit_as_char);
                }
            }
            let number_as_u64 = number_as_str.parse::<u64>()?;
            problem.push(number_as_u64);
        }
        problems.push(problem);
    }

    let grand_total = rev_op
        .iter()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(i, c)| match c {
            '+' => problems[i].iter().sum::<u64>(),
            '*' => problems[i].iter().product::<u64>(),
            _ => unreachable!(),
        })
        .sum::<u64>();

    Ok(grand_total)
}

fn main() -> Result<()> {
    #[cfg(feature = "part_1")]
    println!("Part One: {}", solution_part_1(PUZZLE_INPUT)?);

    #[cfg(feature = "part_2")]
    println!("Part Two: {}", solution_part_2(PUZZLE_INPUT)?);

    Ok(())
}

#[cfg(feature = "part_1")]
#[test]
fn test_part_1() -> Result<()> {
    const EXAMPLE_INPUT_1: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    const EXAMPLE_OUTPUT_1: i64 = 4277556;

    assert_eq!(solution_part_1(EXAMPLE_INPUT_1)?, EXAMPLE_OUTPUT_1);

    Ok(())
}

#[cfg(feature = "part_2")]
#[test]
fn test_part_2() -> Result<()> {
    const EXAMPLE_INPUT_2: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    const EXAMPLE_OUTPUT_2: u64 = 3263827;

    assert_eq!(solution_part_2(EXAMPLE_INPUT_2)?, EXAMPLE_OUTPUT_2);

    Ok(())
}
