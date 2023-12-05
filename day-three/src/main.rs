fn main() {
    part_one();
    // part_two();
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
fn part_two() {}

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
                let segment = Segment::PartNumber {
                    bounds,
                    number: s.parse::<usize>().expect("valid part number"),
                };
                segments.push(segment);
            }
            c => {
                let mut s = String::with_capacity(chars.len());
                s.push(c);
                while let Some((_, c)) = chars.peek() {
                    if c.is_digit(10) || *c == '.' {
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
                let segment = Segment::Symbol { bounds, tokens: s };
                segments.push(segment);
            }
        }
    }

    segments
}

#[derive(Debug)]
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

#[derive(Debug)]
enum Segment {
    Vacant {
        bounds: Bounds,
    },
    PartNumber {
        bounds: Bounds,
        number: usize,
    },
    Symbol {
        bounds: Bounds,
        #[allow(unused)]
        tokens: String,
    },
}

impl Segment {
    pub fn bounds(&self) -> &Bounds {
        match self {
            Segment::Vacant { bounds } => bounds,
            Segment::PartNumber { bounds, .. } => bounds,
            Segment::Symbol { bounds, .. } => bounds,
        }
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol { .. })
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Segment>>,
}

impl Grid {
    pub fn part_numbers(&self) -> impl Iterator<Item = usize> + '_ {
        self.rows.iter().flat_map(|row| {
            row.iter().filter_map(|segment| match segment {
                Segment::PartNumber { bounds, number } => {
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
