use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    // part_one();
    part_two();
}

#[allow(unused)]
fn part_one() {
    // let input = "1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet";

    let input = include_str!("./input.txt");
    let answer: usize = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(calibration_value_p1)
        .sum();
    println!("{answer}");
}

#[allow(unused)]
fn part_two() {
    // let input = "two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen";

    // Guesses:
    // 57325 is too low
    let input = include_str!("./input.txt");

    let trie = {
        let mut t = PrefixTrie::<char>::new();

        t.push_str("one", '1');
        t.push_str("two", '2');
        t.push_str("three", '3');
        t.push_str("four", '4');
        t.push_str("five", '5');
        t.push_str("six", '6');
        t.push_str("seven", '7');
        t.push_str("eight", '8');
        t.push_str("nine", '9');

        t
    };

    let answer: usize = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| calibration_value_p2(s, &trie))
        .sum();
    println!("{answer}");
}

fn calibration_value_p1(line: &str) -> usize {
    let digits = line.chars().filter(|c| c.is_digit(10));
    // clone the iterator in cases where there's only a single digit (counts as both first & last)
    let first = digits.clone().next().expect("at least one digit");
    let last = digits.clone().next_back().expect("at least one digit");
    return format!("{first}{last}").parse().expect("valid number");
}

fn calibration_value_p2(line: &str, trie: &PrefixTrie<char>) -> usize {
    let digits = {
        let mut vec = Vec::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                vec.push(c);
                continue;
            }

            if let Some((digit, _n_chars)) =
                trie.lookup_prefix(vec![c].into_iter().chain(chars.clone()))
            {
                vec.push(digit);
                // This behaviour is a bit confusing; take this line for example:
                //
                //   twone
                //
                // This contains the digits [2, 1]. In other words, a spelled-out
                // digit may overlap a subsequent spelled-out digit, and both
                // should be considered valid. In this case, the last digit
                // on the line is `1`.
                // for _ in 0..(n_chars - 1) {
                //     let _ = chars.next();
                // }
            }
        }

        vec
    };

    // dbg!(line);
    // dbg!(&digits);

    let first = digits.iter().next().expect("at least one digit");
    let last = digits.iter().next_back().expect("at least one digit");
    return format!("{first}{last}").parse().expect("valid number");
}

struct PrefixTrie<T: Copy + Debug> {
    root: HashMap<char, Box<Leaf<T>>>,
}

#[derive(Debug)]
enum Leaf<T: Copy + Debug> {
    Link(HashMap<char, Box<Leaf<T>>>),
    Value(T),
    // prefixes that contain other prefixes aren't allowed; pushing
    // additional prefixes that share a common prefix with an existing
    // key will overwrite that key; i.e., this variant is excluded:
    // LinkAndValue((T, HashMap<char, Box<Leaf<T>>>))
}

impl<T: Copy + Debug> Leaf<T> {
    fn has_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    fn links(&mut self) -> &mut HashMap<char, Box<Leaf<T>>> {
        match self {
            Self::Link(links) => links,
            _ => panic!("tried to access links on a value leaf"),
        }
    }
}

impl<T: Copy + Debug> PrefixTrie<T> {
    pub fn new() -> Self {
        Self {
            root: HashMap::new(),
        }
    }

    pub fn push_str(&mut self, s: &str, value: T) {
        let mut m = &mut self.root;

        let mut chars = s.chars();
        let last_char = chars.next_back();

        for c in chars {
            let leaf = m
                .entry(c)
                .or_insert_with(|| Box::new(Leaf::Link(HashMap::new())));

            if leaf.has_value() {
                panic!("tried to push string that overlaps existing value {value:?}");
            }

            m = leaf.links();
        }

        match last_char {
            None => panic!("expected key to have at least one character"),
            Some(c) => m.insert(c, Box::new(Leaf::Value(value))),
        };
    }

    // `key_chars` is an iterator that starts with a prefix and may contain additional
    // characters after that prefix. If the prefix is found, the corresponding
    // value and the number of characters the prefix contains are returned.
    // If the prefix isn't found, `None` is returned instead.
    pub fn lookup_prefix(&self, key_chars: impl Iterator<Item = char>) -> Option<(T, usize)> {
        let m = &mut &self.root;
        for (i, c) in key_chars.enumerate() {
            match m.get(&c) {
                Some(leaf) => match **leaf {
                    Leaf::Link(ref links) => {
                        *m = links;
                    }
                    Leaf::Value(value) => {
                        return Some((value, i + 1));
                    }
                },
                None => {
                    // the key doesn't match any prefixes
                    return None;
                }
            }
        }

        // we ran out of key characters while traversing the tree,
        // meaning the key is a partial match for at least one value
        return None;
    }
}

impl<T: Copy + Debug> Debug for PrefixTrie<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_links<A: Copy + Debug>(
            f: &mut std::fmt::Formatter<'_>,
            links: &HashMap<char, Box<Leaf<A>>>,
            prefix: &str,
        ) -> std::fmt::Result {
            let mut links = links.iter().collect::<Vec<_>>();
            links.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
            for (c, leaf) in links {
                match **leaf {
                    Leaf::Link(ref links) => fmt_links(f, links, &format!("{prefix}{c}"))?,
                    Leaf::Value(ref value) => writeln!(f, "{prefix}{c} => {value:?},")?,
                }
            }

            Ok(())
        }

        writeln!(f, "PrefixTrie {{")?;
        fmt_links(f, &self.root, "  ")?;
        writeln!(f, "}}")?;

        Ok(())
    }
}
