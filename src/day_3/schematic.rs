#[cfg(test)]
mod tests;

use std::{convert, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right")
        }
    }
}

pub type Location = (usize, usize);

trait LocationMethods {
    fn left(&self) -> Option<Location>;
    fn top_left(&self) -> Option<Location>;
    fn top(&self) -> Option<Location>;
    fn top_right(&self) -> Option<Location>;
    fn right(&self) -> Option<Location>;
    fn bottom_right(&self) -> Option<Location>;
    fn bottom(&self) -> Option<Location>;
    fn bottom_left(&self) -> Option<Location>;
}

impl LocationMethods for Location {
    fn left(&self) -> Option<Location> {
        if self.1 > 0 { Some((self.0, self.1 - 1)) } else { None }
    }
    
    fn top_left(&self) -> Option<Location> {
        if self.0 > 0 && self.1 > 0 { Some((self.0 - 1, self.1 - 1)) } else { None }
    }
    
    fn top(&self) -> Option<Location> {
        if self.0 > 0 { Some((self.0 - 1, self.1)) } else { None }
    }
    
    fn top_right(&self) -> Option<Location> {
        if self.0 > 0 {  Some((self.0 - 1, self.1 + 1)) } else { None }
    }
    
    fn right(&self) -> Option<Location> {
        Some((self.0, self.1 + 1))
    }
    
    fn bottom_right(&self) -> Option<Location> {
        Some((self.0 + 1, self.1 + 1))
    }
    
    fn bottom(&self) -> Option<Location> {
        Some((self.0 + 1, self.1))
    }
    
    fn bottom_left(&self) -> Option<Location> {
        if self.1 > 0 { Some((self.0 + 1, self.1 - 1)) } else { None }
    }
}

#[derive(Debug, PartialEq)]
struct NumberSequence<'a> {
    schematic: &'a Schematic<'a>,
    location: Location,
    direction: Direction
}

impl<'a> NumberSequence<'a> {
    pub fn new(schematic: &'a Schematic<'a>, location: Location, direction: Direction) -> Self {
        Self { schematic, location, direction }
    }
}

impl<'a> Iterator for NumberSequence<'a> {
    type Item = (char, Location);

    fn next(&mut self) -> Option<Self::Item> {
        let next_location = match self.direction {
            Direction::Left => self.location.left(),
            Direction::Right => self.location.right()
        };

        match next_location {
            Some(next_location) => {
                match self.schematic.get_char_at_location(next_location) {
                    Some(char) => {
                        if char.is_digit(10) {
                            self.location = next_location;
                            Some((char, self.location))
                        } else {
                            None
                        }
                    },
                    None => None
                }
            },
            None => None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Schematic<'a> {
    input: Vec<&'a str>
}

impl<'a> Schematic<'a> {
    pub fn new(input: Vec<&'a str>) -> Self {
        Self { input }
    }

    fn enumerate_input_chars(&'a self) -> impl Iterator<Item = (char, Location)> + 'a {
        self.input
        .iter()
        .enumerate()
        .flat_map(
            |(line_index, &line)|
                line.chars()
                    .enumerate()
                    .map(move |(char_index, char)| (char, (line_index, char_index)))
        )
    }

    fn get_char_at_location(&self, location: Location) -> Option<char> {
        self.input.get(location.0).map(|&line| line.chars().nth(location.1)).flatten()
    }

    fn get_surrounding_locations(&'a self, location: Location) -> impl Iterator<Item = Location> + 'a {
        let surrounding_locations: Vec<Option<Location>> =
        if location.0 < self.input.len() &&
            location.1 < self.input.get(location.0).map(|&line| line).unwrap_or_default().len()
        {
            vec!(
                location.left(),
                location.top_left(),
                location.top(),
                location.top_right(),
                location.right(),
                location.bottom_right(),
                location.bottom(),
                location.bottom_left()
            )
        } else {
            vec!()
        };

        surrounding_locations.into_iter()
            .filter_map(convert::identity)
    }

    fn get_surrounding_chars(&'a self, location: Location) -> impl Iterator<Item = char> + 'a {
        self.get_surrounding_locations(location)
            .filter_map(|location| self.get_char_at_location(location))
    }

    fn get_surrounding_digits_with_location(&'a self, location: Location) -> impl Iterator<Item = (char, Location)> + 'a {
        self.get_surrounding_locations(location)
            .filter_map(
                |location|
                    self.get_char_at_location(location)
                        .and_then(
                            |char|
                                if char.is_digit(10) {
                                    Some((char, location))
                                } else {
                                    None
                                }
                        )
            )
    }

    fn get_digits_in_direction(&self, location: Location, direction: Direction) -> Option<(String, Location)> {
        let number_sequence = NumberSequence::new(self, location, direction);
        number_sequence.fold(
            None as Option<(String, Location)>,
            |acc, (char, location)|
                match acc {
                    Some((acc, _)) => match direction {
                        Direction::Left => Some((format!("{char}{acc}"), location)),
                        Direction::Right => Some((format!("{acc}{char}"), location))
                    },
                    None => Some((char.to_string(), location))
                }
        )
    }

    fn get_full_number_with_location(&self, location: Location) -> Option<(String, Location)> {
        self.get_char_at_location(location)
            .and_then(
                |char|
                    if char.is_digit(10) {
                        let right_digits =
                            self.get_digits_in_direction(location, Direction::Right)
                                .map(|(digits, _)| digits)
                                .unwrap_or_default();
                        match self.get_digits_in_direction(location, Direction::Left) {
                            Some((left_digits, left_location)) => {
                                Some((format!("{left_digits}{char}{right_digits}"), left_location))
                            },
                            None => {
                                Some((format!("{char}{right_digits}"), location))
                            }
                        }
                    } else {
                        None
                    }
            )
    }

    fn get_surrounding_part_numbers(&'a self, location: Location) -> impl Iterator<Item = usize> + 'a {
        let mut full_numbers_with_locations =
            self.get_surrounding_digits_with_location(location)
                .filter_map(
                    |(_, location)| self.get_full_number_with_location(location)
                )
                .collect::<Vec<_>>();
        full_numbers_with_locations.sort_by(|(_, a), (_, b)| a.cmp(b));
        full_numbers_with_locations.dedup();
        full_numbers_with_locations.into_iter().filter_map(
            |(full_number, _)| full_number.parse::<usize>().ok()
        )
    }

    fn has_surrounding_symbol(&self, location: Location) -> bool {
        self.get_surrounding_chars(location)
            .any(|char| !char.is_digit(10) && char != '.')
    }

    pub fn get_valid_part_numbers(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.enumerate_input_chars()
        .fold(
            vec!(),
            |mut acc, (char, (line_index, char_index))| {
                if char.is_digit(10) {
                    let last_result = acc.pop();

                    match last_result {
                        Some((ref digits, is_valid, last_index)) => {
                            if char_index == last_index + 1 {
                                acc.push((
                                    format!("{digits}{char}"),
                                    is_valid || self.has_surrounding_symbol((line_index, char_index)),
                                    char_index
                                ));
                            } else {
                                acc.push((digits.to_string(), is_valid, last_index));
                                acc.push((
                                    char.to_string(),
                                    self.has_surrounding_symbol((line_index, char_index)),
                                    char_index
                                ))
                            }
                        }
                        None => {
                            acc.push((
                                char.to_string(),
                                self.has_surrounding_symbol((line_index, char_index)),
                                char_index
                            ))
                        }
                    }
                }
                        
                acc
            }
        )
        .into_iter()
        .filter_map(|(digits, is_valid, _)| {
            if is_valid {
                digits.parse().ok()
            } else {
                None
            }
        })
    }

    pub fn get_gear_ratios(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.enumerate_input_chars()
            .filter(|(char, _)| char.eq(&'*'))
            .map(|(_, gear_location)| {
                self.get_surrounding_part_numbers(gear_location).collect::<Vec<_>>()
            })
            .filter(|part_numbers| part_numbers.len() == 2)
            .map(|part_numbers| part_numbers.into_iter().product())
    }
}
