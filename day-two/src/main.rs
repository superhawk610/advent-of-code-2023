use std::collections::HashMap;

fn main() {
    // part_one();
    part_two();
}

#[allow(unused)]
fn part_one() {
    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let input = include_str!("./input.txt");

    let available_cubes = {
        let mut m: HashMap<&str, usize> = HashMap::new();

        m.insert("red", 12);
        m.insert("green", 13);
        m.insert("blue", 14);

        m
    };

    let game_id_sum = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_game)
        .filter_map(|(game_id, sets)| {
            if sets.iter().all(|set| {
                set.iter()
                    .all(|(color, count)| available_cubes.get(color).unwrap_or(&0) >= count)
            }) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{game_id_sum}");
}

#[allow(unused)]
fn part_two() {
    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let input = include_str!("./input.txt");

    let power_set_sum = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_game)
        .map(|(game_id, sets)| {
            sets.iter()
                .fold(HashMap::<&str, usize>::new(), |acc, set| {
                    set.iter().fold(acc, |mut acc, (color, count)| {
                        acc.entry(color)
                            .and_modify(|e| *e = std::cmp::max(*e, *count))
                            .or_insert(*count);

                        acc
                    })
                })
                .into_values()
                .product::<usize>()
        })
        .sum::<usize>();
    println!("{power_set_sum}");
}

fn parse_game(line: &str) -> (usize, Vec<HashMap<&str, usize>>) {
    // remove `Game ` prefix
    let line = &line[5..];
    let colon_index = line.find(':').expect("valid format includes colon");
    let game_id = line[0..colon_index]
        .parse::<usize>()
        .expect("numeric game ID");

    let mut sets = Vec::new();
    // remove game ID, colon, and trailing space
    let line = &line[(colon_index + 2)..];
    for set_str in line.split("; ") {
        let mut pulls = HashMap::new();
        for pull in set_str.split(", ") {
            let mut pull = pull.split(" ");
            let count = pull
                .next()
                .expect("pull count")
                .parse::<usize>()
                .expect("numeric pull count");
            let color = pull.next().expect("pull color");
            assert_eq!(pull.next(), None);
            pulls.insert(color, count);
        }
        sets.push(pulls);
    }

    (game_id, sets)
}
