use std::collections::HashSet;

fn main() {
    // part_one();
    part_two();
}

#[allow(unused)]
fn part_one() {
    // let input = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";

    let input = include_str!("./input.txt");

    let rows = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(parse_line)
        .collect::<Vec<Vec<Segment>>>();

    let grid = Grid { rows };
    let part_number_sum = grid.part_numbers().sum::<usize>();
    println!("{part_number_sum}");
}

#[allow(unused)]
fn part_two() {
    // let input = "
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";

    let input = include_str!("./input.txt");

    let rows = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(parse_line)
        .collect::<Vec<Vec<Segment>>>();

    let grid = Grid { rows };
    let gear_ratio_sum = grid.gears().sum::<usize>();
    println!("{gear_ratio_sum}");
}

fn parse_line((row, line): (usize, &str)) -> Vec<Segment> {
    let mut chars = line
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .enumerate()
        .peekable();
    let mut segments = Vec::new();

    while let Some((pos, c)) = chars.next() {
        match c {
            '.' => {
                let mut len = 1;
                while let Some((_, '.')) = chars.peek() {
                    let _ = chars.next();
                    len += 1;
                }

                let bounds = Bounds {
                    row,
                    start: pos,
                    len,
                };
                let segment = Segment::Vacant { bounds };
                segments.push(segment);
            }
            c if c.is_digit(10) => {
                let mut s = String::with_capacity(chars.len());
                s.push(c);
                while let Some((_, c)) = chars.peek() {
                    if !c.is_digit(10) || *c == '.' {
                        break;
                    }

                    s.push(*c);
                    let _ = chars.next();
                }

                let bounds = Bounds {
                    row,
                    start: pos,
                    len: s.len(),
                };
                let segment = Segment::Part {
                    bounds,
                    number: s.parse::<usize>().expect("valid part number"),
                };
                segments.push(segment);
            }
            c => {
                // For part one, we created a single segment for any length run
                // of symbols; for part two, however, multiple gears (`*`) may
                // be adjacent to each other, but should still be counted
                // separately, so we'll instead always create separate Symbol
                // segments for each character.
                //
                // let mut s = String::with_capacity(chars.len());
                // s.push(c);
                // while let Some((_, c)) = chars.peek() {
                //     if c.is_digit(10) || *c == '.' {
                //         break;
                //     }

                //     s.push(*c);
                //     let _ = chars.next();
                // }

                let bounds = Bounds {
                    row,
                    start: pos,
                    len: 1,
                };
                let segment = Segment::Symbol { bounds, token: c };
                segments.push(segment);
            }
        }
    }

    segments
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Bounds {
    row: usize,
    start: usize,
    len: usize,
}

impl Bounds {
    pub fn adjacent_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let row = self.row as isize;
        let start = self.start as isize;
        let len = self.len as isize;
        (-1..=len)
            .map(move |offset| (row - 1, start + offset))
            .chain(vec![(row, start - 1)])
            .chain(vec![(row, start + len)])
            .chain((-1..=len).map(move |offset| (row + 1, start + offset)))
            .filter_map(|(row, col)| {
                if row < 0 || col < 0 {
                    None
                } else {
                    Some((row as usize, col as usize))
                }
            })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Segment {
    Vacant { bounds: Bounds },
    Part { bounds: Bounds, number: usize },
    Symbol { bounds: Bounds, token: char },
}

impl Segment {
    pub fn bounds(&self) -> &Bounds {
        match self {
            Segment::Vacant { bounds } => bounds,
            Segment::Part { bounds, .. } => bounds,
            Segment::Symbol { bounds, .. } => bounds,
        }
    }

    pub fn part_number(&self) -> usize {
        if let Self::Part { number, .. } = self {
            *number
        } else {
            panic!("attempted to read part number on non-part segment")
        }
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol { .. })
    }

    pub fn is_part(&self) -> bool {
        matches!(self, Self::Part { .. })
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Segment>>,
}

impl Grid {
    // Returns the gear ratios of all valid gear segments.
    pub fn gears(&self) -> impl Iterator<Item = usize> + '_ {
        self.rows.iter().flat_map(|row| {
            row.iter().filter_map(|segment| match segment {
                Segment::Symbol { bounds, token: '*' } => {
                    let mut adjacent_parts = HashSet::with_capacity(2);
                    for (row, col) in bounds.adjacent_coords() {
                        if let Some(segment) = self.lookup(row, col) {
                            if segment.is_part() && !adjacent_parts.contains(segment) {
                                if adjacent_parts.len() == 2 {
                                    // gear is adjacent to too many parts
                                    return None;
                                }
                                adjacent_parts.insert(segment);
                            }
                        }
                    }

                    if adjacent_parts.len() == 2 {
                        Some(
                            adjacent_parts
                                .iter()
                                .map(|s| s.part_number())
                                .product::<usize>(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
        })
    }

    // Returns the part number for all valid Part segments (adjacent
    // to one or more Symbol segments, including diagonally).
    pub fn part_numbers(&self) -> impl Iterator<Item = usize> + '_ {
        self.rows.iter().flat_map(|row| {
            row.iter().filter_map(|segment| match segment {
                Segment::Part { bounds, number } => {
                    if bounds.adjacent_coords().any(|(row, col)| {
                        self.lookup(row, col)
                            .map(|s| s.is_symbol())
                            .unwrap_or(false)
                    }) {
                        Some(*number)
                    } else {
                        None
                    }
                }
                _ => None,
            })
        })
    }

    pub fn lookup(&self, row: usize, col: usize) -> Option<&Segment> {
        self.rows.get(row).and_then(|row| {
            for segment in row {
                if col < segment.bounds().start + segment.bounds().len {
                    return Some(segment);
                }
            }

            None
        })
    }
}
