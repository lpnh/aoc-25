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
    let list: Vec<Vec<&str>> = worksheet.map(|p| p.split_whitespace().collect()).collect();

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
fn solution_part_2(input: &str) -> Result<String> {
    let result = input
        .lines()
        .next()
        .context("missing first line")?
        .replace("input", "output");

    Ok(result)
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
Part Two example input
";

    const EXAMPLE_OUTPUT_2: &str = "Part Two example output";

    assert_eq!(solution_part_2(EXAMPLE_INPUT_2)?, EXAMPLE_OUTPUT_2);

    Ok(())
}
