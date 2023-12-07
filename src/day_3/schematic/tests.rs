use super::Schematic;

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
    assert_eq!(schematic.get_surrounding_chars((3, 6)).collect::<Vec<_>>(), vec!('.',  '.', '6', '3', '.', '.', '.','.'));
    assert_eq!(schematic.get_surrounding_chars((8, 6)).collect::<Vec<_>>(), vec!('*',  '.', '7', '5', '.', '8', '9','5'));
    assert_eq!(schematic.get_surrounding_chars((5, 10)).collect::<Vec<_>>(), vec!());
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
