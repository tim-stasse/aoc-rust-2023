#[cfg(test)]
mod tests;

use anyhow::{bail, Result, anyhow, Context};
use std::convert;

pub fn solve<'a>(input: impl Iterator<Item = &'a str>) -> Result<usize> {
    let values = input
        .map(|input_value| get_possible_game_id(input_value))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .sum::<usize>();

    Ok(values)
}

fn get_possible_game_id(input: &str) -> Result<Option<usize>> {
    let game = parse_game(input)?;

    Ok(
        if game.is_possible() { Some(game.id) } else { None }
    )
}

fn parse_game(input: &str) -> Result<Game> {
    let (id, sets) = input.split_once(":")
        .ok_or(anyhow!("unable to parse game: {}", input))?;

    Ok(
        Game {
            id: parse_game_id(id)?,
            sets: parse_game_sets(sets)?
        }
    )
}

fn parse_game_id(input: &str) -> Result<usize> {
    static PARSE_GAME_ID_ERROR_MESSAGE: &str = "unable to parse game id";

    match input.split_once(" ") {
        Some((_, id)) => id.parse().with_context(|| format!("{}: {}", PARSE_GAME_ID_ERROR_MESSAGE, input)),
        _ => bail!("{}: {}", PARSE_GAME_ID_ERROR_MESSAGE, input)
    }
}

fn parse_game_sets(input: &str) -> Result<Vec<GameSet>> {
    input.split(";")
        .map(parse_game_set)
        .collect()
}

fn parse_game_set(input: &str) -> Result<GameSet> {
    let cubes = input.split(",")
        .map(parse_cubes)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(
        GameSet {
            red_cubes: cubes.iter().find(|(_, colour)| *colour == Colour::Red).map(|(count, _)| *count),
            green_cubes: cubes.iter().find(|(_, colour)| *colour == Colour::Green).map(|(count, _)| *count),
            blue_cubes: cubes.iter().find(|(_, colour)| *colour == Colour::Blue).map(|(count, _)| *count)
        }
    )
}

fn parse_cubes(input: &str) -> Result<(u8, Colour)> {
    let (count, colour) = input.trim()
        .split_once(" ")
        .ok_or(anyhow!("unable to parse cubes: {}", input))?;

    Ok((count.parse()?, parse_colour(colour)?))
}

fn parse_colour(input: &str) -> Result<Colour> {
    match input.trim() {
        "red" => Ok(Colour::Red),
        "green" => Ok(Colour::Green),
        "blue" => Ok(Colour::Blue),
        input => bail!("unable to parse colour: {}", input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct GameSet {
    red_cubes: Option<u8>,
    green_cubes: Option<u8>,
    blue_cubes: Option<u8>
}

impl GameSet {
    fn is_possible(&self) -> bool {
        static POSSIBLE_RED_CUBES: u8 = 12;
        static POSSIBLE_GREEN_CUBES: u8 = 13;
        static POSSIBLE_BLUE_CUBES: u8 = 14;

        [
            self.red_cubes.is_none() || self.red_cubes.is_some_and(|count| count <= POSSIBLE_RED_CUBES),
            self.green_cubes.is_none() || self.green_cubes.is_some_and(|count| count <= POSSIBLE_GREEN_CUBES),
            self.blue_cubes.is_none() || self.blue_cubes.is_some_and(|count| count <= POSSIBLE_BLUE_CUBES)
        ].into_iter().all(convert::identity)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    sets: Vec<GameSet>
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(GameSet::is_possible)
    }
}
