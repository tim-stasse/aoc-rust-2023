#[cfg(test)]
mod tests;

use anyhow::Result;
use super::game::Game;

pub fn solve<'a>(input: impl Iterator<Item = &'a str>) -> Result<usize> {
    let values = input
        .map(get_possible_game_id)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .sum::<usize>();

    Ok(values)
}

fn get_possible_game_id(input: &str) -> Result<Option<usize>> {
    let game = Game::parse(input)?;

    Ok(
        if game.is_possible() { Some(game.id) } else { None }
    )
}
