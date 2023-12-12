#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::ops::Range;
use super::scratchcard::Scratchcard;

type ScratchcardCopies = HashMap<u32, (u32, u32)>;

pub fn solve<'a>(input: impl Iterator<Item = &'a str>) -> Result<u32> {
    let mut scratchcard_copies = input.map(|line| Scratchcard::parse(line))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|scratchcard| Ok((scratchcard.id, (scratchcard.get_matching_numbers().len().try_into()?, 1 as u32))))
        .collect::<Result<ScratchcardCopies>>()?;

    let mut total_copies = 0;

    for i in 1..scratchcard_copies.len() + 1 {
        let id: u32 = i.try_into()?;
        let &(matches, copies) = scratchcard_copies.get(&id).ok_or(anyhow!("unknown id: {id}"))?;
        for _ in 0..copies {
            for next_id in get_copy_range(id, matches) {
                match scratchcard_copies.get(&next_id) {
                    Some(&(matches, copies)) => {
                        scratchcard_copies.insert(next_id, (matches, copies + 1));
                    },
                    None => ()
                }
            }
        }

        total_copies = total_copies + copies;
    }

    Ok(total_copies)
}

fn get_copy_range(id: u32, matches: u32) -> Range<u32>  {
    id + 1..matches + id + 1
}
