#[cfg(test)]
mod tests;

use anyhow::Result;
use super::game::Game;

pub fn solve<'a>(input: impl Iterator<Item = &'a str>) -> Result<usize> {
    let values = input
        .map(Game::parse)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|game| game.min_possible_set().power_of())
        .sum::<usize>();

    Ok(values)
}
