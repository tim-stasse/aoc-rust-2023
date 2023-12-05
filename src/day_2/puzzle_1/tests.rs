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
