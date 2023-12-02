use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = extract_sum(input);

    dbg!(output);
}

fn extract_sum(input: &str) -> u32 {
    let map_check: BTreeMap<&str, u32> = BTreeMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    input.lines()
        .map(|line| {
            let split_game_and_rounds: Vec<&str> = line.split(":").collect();
            let rounds: Vec<&str> = split_game_and_rounds[1]
                .trim()
                .split("; ")
                .collect();

            let has_valid_rounds: bool = rounds
                .iter()
                .all(|round| {
                    let mut color_map: HashMap<&str, u32> = HashMap::new();

                    let _t: Vec<()> = round.split(", ")
                        .map(|c| {
                            let (amount, color) = c.split_at(c.find(' ').unwrap());

                            *color_map
                                .entry(color.trim())
                                .or_insert(0) += amount.parse::<u32>().expect("Amount should be parsable");
                        }).collect();

                    color_map
                        .iter()
                        .all(|color| {
                            *color.1 <= *map_check
                                .get(color.0)
                                .expect("There should be a definition for this color")
                        })
                });

            if has_valid_rounds {
                split_game_and_rounds[0]
                    .split(' ')
                    .collect::<Vec<&str>>()
                    [1]
                    .parse::<u32>()
                    .expect("Should be a parsable number")
            } else {
                0
            }
        })
        .filter(|id| *id != 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::extract_sum;

    #[test]
    fn it_extracts_the_sum() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, extract_sum(input))
    }

    #[test]
    fn test_with_real_input() {
        let input = "Game 3: 6 red, 3 blue, 8 green; 6 blue, 12 green, 15 red; 3 blue, 18 green, 4 red
Game 4: 1 blue, 4 red; 2 blue, 6 red; 13 blue; 11 blue, 1 green, 8 red; 10 blue, 3 green, 2 red; 3 green, 7 blue";

        assert_eq!(4, extract_sum(input))
    }
}