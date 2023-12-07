use std::collections::HashSet;

// uh-oh, part two feels like we're starting to bump against the
// "you can't reasonably brute-force this" limit :3
fn main() {
    // part_one();
    part_two();
}

#[allow(unused)]
fn part_one() {
    // let input = "
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // ";

    let input = include_str!("./input.txt");

    let total_points = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_card)
        .map(|c| c.points())
        .sum::<usize>();
    println!("{total_points}");
}

#[allow(unused)]
fn part_two() {
    // let input = "
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // ";

    let input = include_str!("./input.txt");

    let cards = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_card)
        .collect::<Vec<_>>();

    let mut copies = vec![1; cards.len()];
    // for each card in the original set...
    for (cursor, card) in cards.iter().enumerate() {
        // for each copy of that card we now have available...
        for _ in 0..copies[cursor] {
            // spawn a new copy of each subsequent card for the
            // number of matches that this card scored
            for copy_offset in 1..(card.matches() + 1) {
                copies[cursor + copy_offset] += 1;
            }
        }
    }
    let num_cards = copies.iter().sum::<usize>();
    println!("{num_cards}");
}

fn parse_card(line: &str) -> Card {
    // trim `Card ` prefix
    let line = &line[5..];
    let colon_index = line.find(':').expect("valid format includes colon");
    let card_number = line[0..colon_index]
        .trim()
        .parse::<usize>()
        .expect("numeric game ID");

    // remove game ID, colon, and trailing space
    let line = &line[(colon_index + 2)..];
    let mut parts = line.split(" | ");
    let need: HashSet<usize> = parse_numbers(parts.next().expect("winning numbers")).collect();
    let have: Vec<usize> = parse_numbers(parts.next().expect("player numbers")).collect();
    assert_eq!(parts.next(), None);

    Card {
        number: card_number,
        need,
        have,
    }
}

fn parse_numbers(s: &str) -> impl Iterator<Item = usize> + '_ {
    // this could be done with a regex for /\s+/, but I'm lazy
    s.split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().expect("valid player number"))
}

#[derive(Debug)]
struct Card {
    #[allow(unused)]
    number: usize,
    need: HashSet<usize>,
    have: Vec<usize>,
}

impl Card {
    fn points(&self) -> usize {
        // let mut score = 0;

        // for n in self.have.iter() {
        //     if self.need.contains(&n) {
        //         if score == 0 {
        //             score = 1;
        //         } else {
        //             score *= 2;
        //         }
        //     }
        // }

        // score

        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            2_usize.pow((matches - 1) as u32)
        }
    }

    fn matches(&self) -> usize {
        self.have.iter().filter(|n| self.need.contains(n)).count()
    }
}
