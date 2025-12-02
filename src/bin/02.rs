use anyhow::{Context, Result};
use std::ops::Range;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_02.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<usize> {
    let ranges: Vec<Range<_>> = input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|r| r.split('-').collect::<Vec<_>>())
        .flat_map(|t| {
            let mut ranges = Vec::<Range<String>>::new();
            filter_even_decimal_ranges(t[0].to_owned(), t[1].to_owned(), &mut ranges);
            ranges
        })
        .collect();

    let res = ranges
        .iter()
        .map(|r| -> Result<usize> {
            let start = r.start.parse::<usize>().context("failed to parse start")?;
            let end = r.end.parse::<usize>().context("failed to parse end")?;
            Ok((start..=end)
                .filter(|n| is_invalid_id(n.to_string()))
                .sum::<usize>())
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    Ok(res)
}

#[cfg(feature = "part_2")]
fn solution_part_2(input: &str) -> Result<usize> {
    let ranges: Vec<Range<_>> = input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|r| r.split('-').collect::<Vec<_>>())
        .map(|t| Range {
            start: t[0],
            end: t[1],
        })
        .collect();

    let res = ranges
        .iter()
        .map(|r| -> Result<usize> {
            let start = r.start.parse::<usize>().context("failed to parse start")?;
            let end = r.end.parse::<usize>().context("failed to parse end")?;
            Ok((start..=end)
                .filter(|n| is_invalid_id_part_2(n.to_string()))
                .sum::<usize>())
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    Ok(res)
}

fn filter_even_decimal_ranges(start: String, end: String, ranges: &mut Vec<Range<String>>) {
    let start_len = start.len();
    let end_len = end.len();

    if start_len > end_len {
        return;
    }

    let start_is_even = start_len.is_multiple_of(2);
    let end_is_even = end_len.is_multiple_of(2);

    if start_len == end_len {
        if start_is_even {
            ranges.push(Range { start, end });
        }

        return;
    }

    if start_is_even && end_is_even {
        ranges.push(Range {
            start,
            end: "9".repeat(start_len),
        });
        let next_start = format!("10{}", "0".repeat(start_len));
        filter_even_decimal_ranges(next_start, end, ranges);
    } else if start_is_even {
        ranges.push(Range {
            start,
            end: "9".repeat(start_len),
        });
        let next_start = format!("10{}", "0".repeat(start_len));
        let next_end = "9".repeat(end_len - 1);
        filter_even_decimal_ranges(next_start, next_end, ranges);
    } else if end_is_even {
        let next_start = format!("1{}", "0".repeat(start_len));
        filter_even_decimal_ranges(next_start, end, ranges);
    } else {
        let next_start = format!("1{}", "0".repeat(start_len));
        let next_end = "9".repeat(end_len - 1);
        filter_even_decimal_ranges(next_start, next_end, ranges);
    }
}

fn is_invalid_id(n: String) -> bool {
    let (first_half, second_half) = n.split_at(n.len() / 2);
    first_half == second_half
}

fn is_invalid_id_part_2(n: String) -> bool {
    let chars: Vec<char> = n.chars().collect();

    for i in 1..=n.len() / 2 {
        let pattern = &n[..i];
        let mut chunks = chars.chunks(i);
        if chunks.all(|c| c.iter().collect::<String>().eq(pattern)) {
            return true;
        }
    }

    false
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
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    const EXAMPLE_OUTPUT_1: usize = 1227775554;

    assert_eq!(solution_part_1(EXAMPLE_INPUT_1)?, EXAMPLE_OUTPUT_1);

    Ok(())
}

#[cfg(feature = "part_2")]
#[test]
fn test_part_2() -> Result<()> {
    const EXAMPLE_INPUT_2: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    const EXAMPLE_OUTPUT_2: usize = 4174379265;

    assert_eq!(solution_part_2(EXAMPLE_INPUT_2)?, EXAMPLE_OUTPUT_2);

    Ok(())
}
