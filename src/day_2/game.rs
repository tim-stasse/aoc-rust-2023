#[cfg(test)]
mod tests;

use anyhow::{anyhow, bail, Context, Result};
use std::convert;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    Red,
    Green,
    Blue,
}

impl Colour {
    pub fn parse(input: &str) -> Result<Colour> {
        match input.trim() {
            "red" => Ok(Colour::Red),
            "green" => Ok(Colour::Green),
            "blue" => Ok(Colour::Blue),
            input => bail!("unable to parse colour: {}", input)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameSet {
    pub red_cubes: Option<u8>,
    pub green_cubes: Option<u8>,
    pub blue_cubes: Option<u8>
}

impl GameSet {
    pub fn parse_sets(input: &str) -> Result<Vec<GameSet>> {
        input.split(";")
            .map(GameSet::parse)
            .collect()
    }

    pub fn parse(input: &str) -> Result<GameSet> {
        let cubes = input.split(",")
            .map(GameSet::parse_cubes)
            .collect::<Result<Vec<_>, _>>()?;
    
        Ok(
            GameSet {
                red_cubes: cubes.iter().find(|(_, colour)| *colour == Colour::Red).map(|(count, _)| *count),
                green_cubes: cubes.iter().find(|(_, colour)| *colour == Colour::Green).map(|(count, _)| *count),
                blue_cubes: cubes.iter().find(|(_, colour)| *colour == Colour::Blue).map(|(count, _)| *count)
            }
        )
    }
    
    pub fn parse_cubes(input: &str) -> Result<(u8, Colour)> {
        let (count, colour) = input.trim()
            .split_once(" ")
            .ok_or(anyhow!("unable to parse cubes: {}", input))?;
    
        Ok((count.parse()?, Colour::parse(colour)?))
    }

    pub fn is_possible(&self) -> bool {
        static POSSIBLE_RED_CUBES: u8 = 12;
        static POSSIBLE_GREEN_CUBES: u8 = 13;
        static POSSIBLE_BLUE_CUBES: u8 = 14;

        [
            self.red_cubes.is_none() || self.red_cubes.is_some_and(|count| count <= POSSIBLE_RED_CUBES),
            self.green_cubes.is_none() || self.green_cubes.is_some_and(|count| count <= POSSIBLE_GREEN_CUBES),
            self.blue_cubes.is_none() || self.blue_cubes.is_some_and(|count| count <= POSSIBLE_BLUE_CUBES)
        ].into_iter().all(convert::identity)
    }

    pub fn power_of(&self) -> usize {
        self.red_cubes.unwrap_or(1) as usize *
        self.green_cubes.unwrap_or(1) as usize *
        self.blue_cubes.unwrap_or(1) as usize
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<GameSet>
}

impl Game {
    pub fn parse(input: &str) -> Result<Game> {
        let (id, sets) = input.split_once(":")
            .ok_or(anyhow!("unable to parse game: {}", input))?;
    
        Ok(
            Game {
                id: Game::parse_id(id)?,
                sets: GameSet::parse_sets(sets)?
            }
        )
    }
    
    pub fn parse_id(input: &str) -> Result<usize> {
        static PARSE_GAME_ID_ERROR_MESSAGE: &str = "unable to parse game id";
    
        match input.split_once(" ") {
            Some((_, id)) => id.parse().with_context(|| format!("{}: {}", PARSE_GAME_ID_ERROR_MESSAGE, input)),
            _ => bail!("{}: {}", PARSE_GAME_ID_ERROR_MESSAGE, input)
        }
    }

    pub fn is_possible(&self) -> bool {
        self.sets.iter().all(GameSet::is_possible)
    }

    pub fn min_possible_set(&self) -> GameSet {
        GameSet {
            red_cubes: self.sets.iter().map(|set| set.red_cubes.unwrap_or_default()).max_by(|x, y| x.cmp(y)),
            green_cubes: self.sets.iter().map(|set| set.green_cubes.unwrap_or_default()).max_by(|x, y| x.cmp(y)),
            blue_cubes: self.sets.iter().map(|set| set.blue_cubes.unwrap_or_default()).max_by(|x, y| x.cmp(y))
        }
    }
}
