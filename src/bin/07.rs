use anyhow::{Context, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_07.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<i32> {
    let diagram = Diagram::from_input(input)?;

    let mut beam_path = vec![vec!['.'; diagram.width]; diagram.height];
    beam_path[0][diagram.start] = '|';

    let mut split_count = 0;

    for (i, line) in diagram.lines.iter().enumerate().skip(1) {
        beam_path[i] = beam_path[i - 1].clone();

        if let Some(splitters) = &line.splitters {
            for &splitter in splitters {
                if beam_path[i - 2][splitter] == '|' {
                    split_count += 1;
                    beam_path[i][splitter] = '.';
                    beam_path[i][splitter - 1] = '|';
                    beam_path[i][splitter + 1] = '|';
                }
            }
        }
    }

    Ok(split_count)
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

#[derive(Debug)]
struct Diagram {
    width: usize,
    height: usize,
    start: usize,
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Line {
    splitters: Option<Vec<usize>>,
}

impl Line {
    fn from_input_line(line: &&str) -> Self {
        let splitters: Vec<usize> = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '^' => Some(i),
                _ => None,
            })
            .collect();

        Self {
            splitters: if !splitters.is_empty() {
                Some(splitters)
            } else {
                None
            },
        }
    }
}

impl Diagram {
    fn from_input(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input.lines().collect();
        let first_line = lines.first().context("missing first line")?;

        Ok(Self {
            width: first_line.len(),
            height: lines.len(),
            start: first_line
                .chars()
                .position(|c| c == 'S')
                .context("missing start point")?,
            lines: lines.iter().map(Line::from_input_line).collect(),
        })
    }
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    const EXAMPLE_OUTPUT_1: i32 = 21;

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
