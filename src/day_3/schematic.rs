#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Schematic<'a> {
    input: Vec<&'a str>
}

impl<'a> Schematic<'a> {
    pub fn new(input: Vec<&'a str>) -> Self {
        Self { input }
    }

    fn get_char_at_location(&'a self, location: (usize, usize)) -> Option<char> {
        self.input.get(location.0).map(|&line| line.chars().nth(location.1)).flatten()
    }

    fn get_surrounding_chars(&'a self, location: (usize, usize)) -> impl Iterator<Item = char> + 'a {
        let surrounding_locations: Vec<Option<(usize, usize)>> =
        if location.0 < self.input.len() &&
            location.1 < self.input.get(location.0).map(|&line| line).unwrap_or_default().len()
        {
            vec!(
                
                if location.1 > 0 { Some((location.0, location.1 - 1)) } else { None }, // left
                if location.0 > 0 && location.1 > 0 { Some((location.0 - 1, location.1 - 1)) } else { None }, // top-left
                if location.0 > 0 { Some((location.0 - 1, location.1)) } else { None }, // top
                if location.0 > 0 {  Some((location.0 - 1, location.1 + 1)) } else { None }, // top-right
                Some((location.0, location.1 + 1)), // right
                Some((location.0 + 1, location.1 + 1)), // bottom-right
                Some((location.0 + 1, location.1)), // bottom
                if location.1 > 0 { Some((location.0 + 1, location.1 - 1)) } else { None }, // bottom-left
            )
        } else {
            vec!()
        };

        surrounding_locations.into_iter()
         .filter_map(
            |location|
                location.map(|location| self.get_char_at_location(location)).flatten()
        )
    }

    fn has_surrounding_symbol(&'a self, location: (usize, usize)) -> bool {
        self.get_surrounding_chars(location)
            .any(|char| !char.is_digit(10) && char != '.')
    }

    pub fn get_valid_part_numbers(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.input
        .iter()
        .enumerate()
        .flat_map(
            |(line_index, &line)|
                line.chars()
                    .enumerate()
                    .map(|(char_index, char)| (char_index, char))
                    .fold(
                        vec!() as Vec<(String, bool, usize)>,
                        |mut acc, (char_index, char)| {
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
        )
    }
}
