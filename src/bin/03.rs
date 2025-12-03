use anyhow::{Context, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_03.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<i32> {
    let total_joltage = input
        .lines()
        .map(max_joltage)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum::<i32>();

    Ok(total_joltage)
}

#[cfg(feature = "part_2")]
fn solution_part_2(input: &str) -> Result<u64> {
    let total_jotage = input
        .lines()
        .map(max_joltage_part_2)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<u64>();

    Ok(total_jotage)
}

fn max_joltage(bank: &str) -> Result<i32> {
    let batteries: Vec<char> = bank.chars().collect();
    let mut max_joltage = 0;

    for i in 0..bank.len() {
        for j in 1 + i..bank.len() {
            let joltage = format!("{}{}", batteries[i], batteries[j])
                .parse::<i32>()
                .context("failed to parse joltage")?;
            if joltage > max_joltage {
                max_joltage = joltage
            }
        }
    }

    Ok(max_joltage)
}

fn max_joltage_part_2(bank: &str) -> Result<u64> {
    let batteries: Vec<char> = bank.chars().collect();
    let mut max_joltages = Vec::new();

    let mut remaining_batteries = batteries.len() - 12;

    for joltage in &batteries {
        while remaining_batteries > 0
            && let Some(last_joltage) = max_joltages.last()
        {
            if joltage > last_joltage {
                max_joltages.pop();
                remaining_batteries -= 1;
            } else {
                break;
            }
        }

        max_joltages.push(*joltage);
    }

    max_joltages.truncate(12);

    let max_joltage_u64 = max_joltages.iter().collect::<String>().parse::<u64>()?;

    Ok(max_joltage_u64)
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
987654321111111
811111111111119
234234234234278
818181911112111
";

    const EXAMPLE_OUTPUT_1: i32 = 357;

    assert_eq!(solution_part_1(EXAMPLE_INPUT_1)?, EXAMPLE_OUTPUT_1);

    Ok(())
}

#[cfg(feature = "part_2")]
#[test]
fn test_part_2() -> Result<()> {
    const EXAMPLE_INPUT_2: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    const EXAMPLE_OUTPUT_2: u64 = 3121910778619;

    assert_eq!(solution_part_2(EXAMPLE_INPUT_2)?, EXAMPLE_OUTPUT_2);

    Ok(())
}
