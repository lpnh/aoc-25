use anyhow::{Context, Result};

const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_04.txt");

#[cfg(feature = "part_1")]
fn solution_part_1(input: &str) -> Result<usize> {
    let diagram = Diagram::from_input(input)?;

    Ok(diagram.rolls_of_paper_that_can_be_accessed())
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
    lines: Vec<Vec<Position>>,
}

impl Diagram {
    fn from_input(input: &str) -> Result<Self> {
        Ok(Self {
            width: input.find('\n').context("fail to find first line ending")?,
            height: input.lines().count(),
            lines: input.lines().enumerate().map(Position::from_line).collect(),
        })
    }

    fn rolls_of_paper_that_can_be_accessed(&self) -> usize {
        self.lines
            .iter()
            .flatten()
            .filter(|p| p.contains_roll_of_paper && p.can_be_accessed(self))
            .count()
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
    contains_roll_of_paper: bool,
}

impl Position {
    fn from_line(line: (usize, &str)) -> Vec<Position> {
        line.1
            .chars()
            .enumerate()
            .map(|(x, char)| Position {
                x,
                y: line.0,
                contains_roll_of_paper: char == '@',
            })
            .collect()
    }

    fn adjacent_positions(&self, diagram: &Diagram) -> [Option<(usize, usize)>; 8] {
        let positions = [
            // Above
            (-1, -1),
            (0, -1),
            (1, -1),
            // Left, Right
            (-1, 0),
            (1, 0),
            // Bellow
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        positions.map(|(px, py)| {
            let x = self.x.checked_add_signed(px)?;
            let y = self.y.checked_add_signed(py)?;

            if x >= diagram.width || y >= diagram.height {
                return None;
            }

            Some((x, y))
        })
    }

    fn can_be_accessed(&self, diagram: &Diagram) -> bool {
        let mut rolls_in_adjacent_position = 0;

        self.adjacent_positions(diagram)
            .iter()
            .flatten()
            .all(|&(px, py)| {
                if diagram.lines[py][px].contains_roll_of_paper {
                    rolls_in_adjacent_position += 1;
                }
                rolls_in_adjacent_position < 4
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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    const EXAMPLE_OUTPUT_1: usize = 13;

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
