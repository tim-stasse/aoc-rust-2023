use super::{Colour, Game, GameSet};

#[test]
fn parse_game() {
    let result1 = Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(
        result1.unwrap(),
        Game {
            id: 1,
            sets: vec!(
                GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) },
                GameSet { red_cubes: Some(1), green_cubes: Some(2), blue_cubes: Some(6) },
                GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: None }
            )
        }
    );
    let result2 = Game::parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    assert_eq!(
        result2.unwrap(),
        Game {
            id: 2,
            sets: vec!(
                GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: Some(1) },
                GameSet { red_cubes: Some(1), green_cubes: Some(3), blue_cubes: Some(4) },
                GameSet { red_cubes: None, green_cubes: Some(1), blue_cubes: Some(1) }
            )
        }
    );
}

#[test]
fn parse_game_id() {
    let result1 = Game::parse_id("Game 1");
    assert_eq!(result1.unwrap(), 1);
    let result2 = Game::parse_id("Game 2");
    assert_eq!(result2.unwrap(), 2);
}

#[test]
fn parse_game_sets() {
    let result1 = GameSet::parse_sets("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(
        result1.unwrap(),
        vec!(
            GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) },
            GameSet { red_cubes: Some(1), green_cubes: Some(2), blue_cubes: Some(6) },
            GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: None }
        )
    );
    let result2 = GameSet::parse_sets("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    assert_eq!(
        result2.unwrap(),
        vec!(
            GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: Some(1) },
            GameSet { red_cubes: Some(1), green_cubes: Some(3), blue_cubes: Some(4) },
            GameSet { red_cubes: None, green_cubes: Some(1), blue_cubes: Some(1) }
        )
    );
}

#[test]
fn parse_game_set() {
    let result1 = GameSet::parse("3 blue, 4 red");
    assert_eq!(result1.unwrap(), GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) });
    let result2 = GameSet::parse("1 red, 2 green, 6 blue");
    assert_eq!(result2.unwrap(), GameSet { red_cubes: Some(1), green_cubes: Some(2), blue_cubes: Some(6) });
    let result3 = GameSet::parse("2 green");
    assert_eq!(result3.unwrap(), GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: None });
    let result4 = GameSet::parse("1 blue, 2 green");
    assert_eq!(result4.unwrap(), GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: Some(1) });
    let result4 = GameSet::parse("3 green, 4 blue, 1 red");
    assert_eq!(result4.unwrap(), GameSet { red_cubes: Some(1), green_cubes: Some(3), blue_cubes: Some(4) });
    let result4 = GameSet::parse("1 green, 1 blue");
    assert_eq!(result4.unwrap(), GameSet { red_cubes: None, green_cubes: Some(1), blue_cubes: Some(1) });
    let result5 = GameSet::parse("1 red, 1 blue, 2 purple");
    assert_eq!(result5.is_err(), true);
}

#[test]
fn parse_cubes() {
    let result1 = GameSet::parse_cubes("3 blue");
    assert_eq!(result1.unwrap(), (3, Colour::Blue));
    let result2 = GameSet::parse_cubes("4 red");
    assert_eq!(result2.unwrap(), (4, Colour::Red));
    let result3 = GameSet::parse_cubes("2 green");
    assert_eq!(result3.unwrap(), (2, Colour::Green));
    let result4 = GameSet::parse_cubes("5 purple");
    assert_eq!(result4.is_err(), true);
}

#[test]
fn parse_colour() {
    let result1 = Colour::parse("blue");
    assert_eq!(result1.unwrap(), Colour::Blue);
    let result2 = Colour::parse("red");
    assert_eq!(result2.unwrap(), Colour::Red);
    let result3 = Colour::parse("green");
    assert_eq!(result3.unwrap(), Colour::Green);
    let result4 = Colour::parse("purple");
    assert_eq!(result4.is_err(), true);
}

#[test]
fn game_set_is_possible() {
    let result1 = GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) }.is_possible();
    assert_eq!(result1, true);
    let result2 = GameSet { red_cubes: Some(4), green_cubes: Some(13), blue_cubes: Some(5) }.is_possible();
    assert_eq!(result2, true);
    let result3 = GameSet { red_cubes: Some(14), green_cubes: Some(3), blue_cubes: Some(15) }.is_possible();
    assert_eq!(result3, false);
}

#[test]
fn game_set_power_of() {
    let result1 = GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) }.power_of();
    assert_eq!(result1, 12);
    let result2 = GameSet { red_cubes: Some(4), green_cubes: Some(13), blue_cubes: Some(5) }.power_of();
    assert_eq!(result2, 260);
    let result3 = GameSet { red_cubes: Some(14), green_cubes: Some(3), blue_cubes: Some(15) }.power_of();
    assert_eq!(result3, 630);
}

#[test]
fn game_is_possible() {
    let result1 = Game {
        id: 1,
        sets: vec!(
            GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) },
            GameSet { red_cubes: Some(1), green_cubes: Some(2), blue_cubes: Some(6) },
            GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: None }
        )
    }.is_possible();
    assert_eq!(result1, true);
    let result2 = Game {
        id: 4,
        sets: vec!(
            GameSet { red_cubes: Some(3), green_cubes: Some(1), blue_cubes: Some(6) },
            GameSet { red_cubes: Some(6), green_cubes: Some(3), blue_cubes: None },
            GameSet { red_cubes: Some(14), green_cubes: Some(3), blue_cubes: Some(15) }
        )
    }.is_possible();
    assert_eq!(result2, false);
}

#[test]
fn min_possible_set() {
    let result1 = Game {
        id: 1,
        sets: vec!(
            GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) },
            GameSet { red_cubes: Some(1), green_cubes: Some(2), blue_cubes: Some(6) },
            GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: None }
        )
    }.min_possible_set();
    assert_eq!(result1, GameSet { red_cubes: Some(4), green_cubes: Some(2), blue_cubes: Some(6) });
    let result2 = Game {
        id: 4,
        sets: vec!(
            GameSet { red_cubes: Some(3), green_cubes: Some(1), blue_cubes: Some(6) },
            GameSet { red_cubes: Some(6), green_cubes: Some(3), blue_cubes: None },
            GameSet { red_cubes: Some(14), green_cubes: Some(3), blue_cubes: Some(15) }
        )
    }.min_possible_set();
    assert_eq!(result2, GameSet { red_cubes: Some(14), green_cubes: Some(3), blue_cubes: Some(15) });
}
