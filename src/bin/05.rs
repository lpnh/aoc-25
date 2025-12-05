use anyhow::{Context, Result};
use std::ops::RangeInclusive;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_05.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<usize> {
    let mut database = input.split("\n\n");

    let ranges: Vec<RangeInclusive<usize>> = database
        .next()
        .context("failed to get ranges")?
        .lines()
        .map(|r| -> Result<RangeInclusive<usize>> {
            let mut range = r
                .split('-')
                .map(|s| s.parse::<usize>().context("failed to parse range"));

            Ok(RangeInclusive::new(
                range.next().context("missing start")??,
                range.next().context("missing end")??,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let ingredients: Vec<usize> = database
        .next()
        .context("failed to get ingredients")?
        .lines()
        .map(|id| -> Result<usize> { id.parse::<usize>().context("failed to parse range") })
        .collect::<Result<Vec<_>>>()?;

    let fresh_count = ingredients
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    Ok(fresh_count)
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    const EXAMPLE_OUTPUT_1: usize = 3;

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
