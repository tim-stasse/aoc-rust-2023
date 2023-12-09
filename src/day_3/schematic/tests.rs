use super::{Direction, Schematic};

#[test]
fn get_char_at_location() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(schematic.get_char_at_location((0, 0)), Some('4'));
    assert_eq!(schematic.get_char_at_location((3, 6)), Some('#'));
    assert_eq!(schematic.get_char_at_location((8, 6)), Some('.'));
    assert_eq!(schematic.get_char_at_location((5, 10)), None);
}

#[test]
fn get_surrounding_locations() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(
        schematic.get_surrounding_locations((0, 0)).collect::<Vec<_>>(),
        vec!((0, 1), (1, 1), (1, 0))
    );
    assert_eq!(
        schematic.get_surrounding_locations((0, 2)).collect::<Vec<_>>(),
        vec!((0, 1), (0, 3), (1, 3), (1, 2), (1, 1))
    );
    assert_eq!(
        schematic.get_surrounding_locations((3, 6)).collect::<Vec<_>>(),
        vec!((3, 5),  (2, 5), (2, 6), (2, 7), (3, 7), (4, 7), (4, 6), (4, 5))
    );
    assert_eq!(
        schematic.get_surrounding_locations((8, 6)).collect::<Vec<_>>(),
        vec!((8, 5),  (7, 5), (7, 6), (7, 7), (8, 7), (9, 7), (9, 6), (9, 5))
    );
    assert_eq!(
        schematic.get_surrounding_locations((5, 10)).collect::<Vec<_>>(),
        vec!()
    );
}

#[test]
fn get_surrounding_chars() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(schematic.get_surrounding_chars((0, 0)).collect::<Vec<_>>(), vec!('6', '.', '.'));
    assert_eq!(schematic.get_surrounding_chars((0, 2)).collect::<Vec<_>>(), vec!('6', '.', '*', '.', '.'));
    assert_eq!(schematic.get_surrounding_chars((3, 6)).collect::<Vec<_>>(), vec!('.', '.', '6', '3', '.', '.', '.', '.'));
    assert_eq!(schematic.get_surrounding_chars((8, 6)).collect::<Vec<_>>(), vec!('*', '.', '7', '5', '.', '8', '9', '5'));
    assert_eq!(schematic.get_surrounding_chars((5, 10)).collect::<Vec<_>>(), vec!());
}

#[test]
fn get_surrounding_digits_with_location() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(
        schematic.get_surrounding_digits_with_location((0, 0)).collect::<Vec<_>>(),
        vec!(('6', (0, 1)))
    );
    assert_eq!(
        schematic.get_surrounding_digits_with_location((0, 2)).collect::<Vec<_>>(),
        vec!(('6', (0, 1)))
    );
    assert_eq!(
        schematic.get_surrounding_digits_with_location((3, 6)).collect::<Vec<_>>(),
        vec!(('6', (2, 6)), ('3', (2, 7)))
    );
    assert_eq!(
        schematic.get_surrounding_digits_with_location((8, 6)).collect::<Vec<_>>(),
        vec!(('7', (7, 6)), ('5', (7, 7)), ('8', (9, 7)), ('9', (9, 6)), ('5', (9, 5)))
    );
    assert_eq!(
        schematic.get_surrounding_digits_with_location((5, 10)).collect::<Vec<_>>(),
        vec!()
    );
}

#[test]
fn get_digits_in_direction() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );

    assert_eq!(schematic.get_digits_in_direction((0, 2), Direction::Left), Some(("46".to_string(), (0, 0))));
    assert_eq!(schematic.get_digits_in_direction((0, 2), Direction::Right), None);

    assert_eq!(schematic.get_digits_in_direction((7, 7), Direction::Left), Some(("7".to_string(), (7, 6))));
    assert_eq!(schematic.get_digits_in_direction((7, 7), Direction::Right), Some(("5".to_string(), (7, 8))));

    assert_eq!(schematic.get_digits_in_direction((9, 5), Direction::Left), None);
    assert_eq!(schematic.get_digits_in_direction((9, 5), Direction::Right), Some(("98".to_string(), (9, 7))));
}

#[test]
fn get_full_number_with_location() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(schematic.get_full_number_with_location((0, 2)), Some(("467".to_string(), (0, 0))));
    assert_eq!(schematic.get_full_number_with_location((7, 7)), Some(("755".to_string(), (7, 6))));
    assert_eq!(schematic.get_full_number_with_location((9, 5)), Some(("598".to_string(), (9, 5))));
}

#[test]
fn get_surrounding_part_numbers() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(
        schematic.get_surrounding_part_numbers((1, 3)).collect::<Vec<_>>(),
        vec!(467, 35)
    );
    assert_eq!(
        schematic.get_surrounding_part_numbers((4, 3)).collect::<Vec<_>>(),
        vec!(617)
    );
    assert_eq!(
        schematic.get_surrounding_part_numbers((8, 6)).collect::<Vec<_>>(),
        vec!(755, 598)
    );
}

#[test]
fn has_surrounding_symbol() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(schematic.has_surrounding_symbol((0, 0)), false);
    assert_eq!(schematic.has_surrounding_symbol((0, 2)), true);
    assert_eq!(schematic.has_surrounding_symbol((3, 6)), false);
    assert_eq!(schematic.has_surrounding_symbol((8, 6)), true);
    assert_eq!(schematic.has_surrounding_symbol((5, 10)), false);
}

#[test]
fn get_valid_part_numbers() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(
        schematic.get_valid_part_numbers().collect::<Vec<_>>(),
        vec!(467, 35, 633, 617, 592, 755, 664, 598)
    );
}

#[test]
fn get_gear_ratios() {
    let schematic = Schematic::new(
        vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."
        )
    );
    assert_eq!(
        schematic.get_gear_ratios().collect::<Vec<_>>(),
        vec!(16345, 451490)
    );
}
