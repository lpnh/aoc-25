use anyhow::{Context, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_01.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<i32> {
    let mut dial = 50;

    let rotations = input
        .lines()
        .map(|l| {
            let mut rotation = l.chars();
            let direction = rotation.next().context("empty line")?;
            let distance = rotation.collect::<String>().parse::<i32>()?;
            Ok((direction, distance))
        })
        .collect::<Result<Vec<(char, i32)>>>()?;

    let pointings: Vec<i32> = rotations
        .iter()
        .map(|(direction, distance)| {
            if *direction == 'L' {
                dial += 100 - (distance % 100);
            } else if *direction == 'R' {
                dial += distance;
            }

            dial % 100
        })
        .collect();

    let at_zero_count = pointings.iter().filter(|i| **i == 0).count() as i32;

    Ok(at_zero_count)
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    const EXAMPLE_OUTPUT_1: i32 = 3;

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
