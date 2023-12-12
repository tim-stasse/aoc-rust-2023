#[cfg(test)]
mod tests;

use anyhow::Result;
use super::scratchcard::Scratchcard;

pub fn solve<'a>(input: impl Iterator<Item = &'a str>) -> Result<u32> {
    Ok(
        input.map(|line| Scratchcard::parse(line))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|scratchcard| scratchcard.get_points())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<u32>()
    )
}
