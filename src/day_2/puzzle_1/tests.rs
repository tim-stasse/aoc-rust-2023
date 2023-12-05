use super::{Colour, Game, GameSet};

#[test]
fn solve() {
    let result = super::solve(
        vec!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ).into_iter()
    );
    assert_eq!(result.unwrap(), 8);
}

#[test]
fn get_possible_game_id() {
    let result1 = super::get_possible_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(result1.unwrap(), Some(1));
    let result2 = super::get_possible_game_id("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    assert_eq!(result2.unwrap(), Some(2));
    let result3 = super::get_possible_game_id("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    assert_eq!(result3.unwrap(), None);
    let result4 = super::get_possible_game_id("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    assert_eq!(result4.unwrap(), None);
    let result5 = super::get_possible_game_id("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert_eq!(result5.unwrap(), Some(5));
}

#[test]
fn parse_game() {
    let result1 = super::parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
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
    let result2 = super::parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
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
    let result1 = super::parse_game_id("Game 1");
    assert_eq!(result1.unwrap(), 1);
    let result2 = super::parse_game_id("Game 2");
    assert_eq!(result2.unwrap(), 2);
}

#[test]
fn parse_game_sets() {
    let result1 = super::parse_game_sets("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(
        result1.unwrap(),
        vec!(
            GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) },
            GameSet { red_cubes: Some(1), green_cubes: Some(2), blue_cubes: Some(6) },
            GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: None }
        )
    );
    let result2 = super::parse_game_sets("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
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
    let result1 = super::parse_game_set("3 blue, 4 red");
    assert_eq!(result1.unwrap(), GameSet { red_cubes: Some(4), green_cubes: None, blue_cubes: Some(3) });
    let result2 = super::parse_game_set("1 red, 2 green, 6 blue");
    assert_eq!(result2.unwrap(), GameSet { red_cubes: Some(1), green_cubes: Some(2), blue_cubes: Some(6) });
    let result3 = super::parse_game_set("2 green");
    assert_eq!(result3.unwrap(), GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: None });
    let result4 = super::parse_game_set("1 blue, 2 green");
    assert_eq!(result4.unwrap(), GameSet { red_cubes: None, green_cubes: Some(2), blue_cubes: Some(1) });
    let result4 = super::parse_game_set("3 green, 4 blue, 1 red");
    assert_eq!(result4.unwrap(), GameSet { red_cubes: Some(1), green_cubes: Some(3), blue_cubes: Some(4) });
    let result4 = super::parse_game_set("1 green, 1 blue");
    assert_eq!(result4.unwrap(), GameSet { red_cubes: None, green_cubes: Some(1), blue_cubes: Some(1) });
    let result5 = super::parse_game_set("1 red, 1 blue, 2 purple");
    assert_eq!(result5.is_err(), true);
}

#[test]
fn parse_cubes() {
    let result1 = super::parse_cubes("3 blue");
    assert_eq!(result1.unwrap(), (3, Colour::Blue));
    let result2 = super::parse_cubes("4 red");
    assert_eq!(result2.unwrap(), (4, Colour::Red));
    let result3 = super::parse_cubes("2 green");
    assert_eq!(result3.unwrap(), (2, Colour::Green));
    let result4 = super::parse_cubes("5 purple");
    assert_eq!(result4.is_err(), true);
}

#[test]
fn parse_colour() {
    let result1 = super::parse_colour("blue");
    assert_eq!(result1.unwrap(), Colour::Blue);
    let result2 = super::parse_colour("red");
    assert_eq!(result2.unwrap(), Colour::Red);
    let result3 = super::parse_colour("green");
    assert_eq!(result3.unwrap(), Colour::Green);
    let result4 = super::parse_colour("purple");
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
