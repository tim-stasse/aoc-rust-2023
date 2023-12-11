#[cfg(test)]
mod tests;
use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug, PartialEq)]
pub struct Scratchcard {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>
}

impl Scratchcard {
    pub fn parse(input: &str) -> Result<Self> {
        static PARSE_SCRATCHCARD_ERROR_MESSAGE: &str = "unable to parse scratchcard";
        let (id, numbers) = input.split_once(":")
            .ok_or(anyhow!("{PARSE_SCRATCHCARD_ERROR_MESSAGE}: {input}"))?;
        let (winning_numbers, numbers_you_have) = numbers.split_once("|")
            .ok_or(anyhow!("{PARSE_SCRATCHCARD_ERROR_MESSAGE}: {input}"))?;
    
        Ok(
            Self::new(
                Self::parse_id(id)?,
                Self::parse_numbers(winning_numbers)?,
                Self::parse_numbers(numbers_you_have)?
            )
        )
    }

    fn parse_id(input: &str) -> Result<u32> {
        static PARSE_SCRATCHCARD_ID_ERROR_MESSAGE: &str = "unable to parse scratchcard id";
        match input.split_whitespace().collect::<Vec<_>>()[..] {
            [_, id] => id.parse().with_context(|| format!("{PARSE_SCRATCHCARD_ID_ERROR_MESSAGE}: {input}")),
            _ => bail!("{PARSE_SCRATCHCARD_ID_ERROR_MESSAGE}: {input}")
        }
    }

    fn parse_numbers(input: &str) -> Result<Vec<u32>> {
        static PARSE_SCRATCHCARD_NUMBERS_ERROR_MESSAGE: &str = "unable to parse scratchcard numbers";
        input.split_whitespace()
             .map(
                |number|
                    number.parse::<u32>()
                          .with_context(|| format!("{}: {}", PARSE_SCRATCHCARD_NUMBERS_ERROR_MESSAGE, input))
            )
             .collect::<Result<Vec<_>, _>>()
    }

    pub fn new(id: u32, winning_numbers: Vec<u32>, numbers_you_have: Vec<u32>) -> Self {
        Self { id, winning_numbers, numbers_you_have }
    }

    pub fn get_points(&self) -> Result<u32> {
        let base: u32 = 2;
        let exp = self.numbers_you_have
            .iter()
            .filter(|number_you_have| self.winning_numbers.contains(number_you_have))
            .collect::<Vec<_>>()
            .len()
            .try_into()?;

        Ok(base.pow(exp) / base)
    }
}
