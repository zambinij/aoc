use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, line_ending,
    },
    IResult,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

struct Game<'a> {
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    fn minimum_cube_set(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();
        self.rounds
            .iter()
            .fold(map, |mut acc, round| {
                for cube in round.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| {
                            *v = (*v).max(cube.amount);
                        })
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
    }
}

// 4 red
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) =
        separated_pair(complete::u32, tag(" "), alpha1)(
            input,
        )?;
    Ok((input, Cube { color, amount }))
}

// 3 blue, 4 red
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) =
        separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, _id) =
        preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(
        tag(": "),
        separated_list1(tag("; "), round),
    )(input)?;
    Ok((input, Game { rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) =
        separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn parse(input: &str) -> u32 {
    let games = parse_games(input).expect("should parse");

    games
        .1
        .iter()
        .map(|game| game.minimum_cube_set())
        .sum::<u32>()
}

fn main() {
    let input = include_str!("./input2.txt");
    let ouput = parse(input);

    dbg!(ouput);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_expected_power() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, parse(input));
    }
}