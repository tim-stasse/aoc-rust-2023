#[cfg(test)]
mod tests;

use anyhow::Result;
use super::schematic::Schematic;

pub fn solve<'a>(input: impl Iterator<Item = &'a str>) -> Result<usize> {
    let values = Schematic::new(input.collect())
        .get_valid_part_numbers()
        .sum::<usize>();

    Ok(values)
}
