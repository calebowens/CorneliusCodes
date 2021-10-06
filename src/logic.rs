use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;
use std::option::Option;

use log::info;

use crate::{Battlesnake, Board, Coord, Game};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn to_str(self: Self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right"
        }
    }
}

pub fn get_info() -> JsonValue {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    json!({
        "apiversion": "1",
        "author": "ChaelCodes",
        "color": "#F09383",
        "head": "bendr",
        "tail": "round-bum",
    })
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, me: &Battlesnake) -> &'static str {
    
    let direction =
        if let Some(chosen) = find_perfect_move(&me, &board) {
            // Corny is being a clever boy and found a perfect move!
            chosen
        } else if let Some(chosen) = find_heuristic_move(&me, &board) {
            // Corny will try a more desperate stratergy that may or may not succeed
            chosen
        } else {
            // When all else fails, corny likes left. What can I say?
            Direction::Left
        }.to_str();
    
    info!("{} MOVE {}", game.id, direction);

    direction
}

// TODO: Write tests
fn find_perfect_move(me: &Battlesnake, board: &Board) -> Option<Direction> {
    let mut possible_moves = HashMap::new();

    let head = me.head;

    // Form a list valid moves for corny to pick from
    possible_moves.insert(Direction::Left, valid_move(&head.left(), &board, &me));
    possible_moves.insert(Direction::Right, valid_move(&head.right(), &board, &me));
    possible_moves.insert(Direction::Up, valid_move(&head.up(), &board, &me));
    possible_moves.insert(Direction::Down, valid_move(&head.down(), &board, &me));

    // TODO: Tell corny how to find food.

    // Corny picks a direction
    // TODO: Encourage corny to use an advanced stratergy rather than their d4.
    let moves = possible_moves
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    moves.choose(&mut rand::thread_rng()).map(|direction: &Direction| direction.clone()) // Chose method from SliceRandom
}

fn find_heuristic_move(me: &Battlesnake, board: &Board) -> Option<Direction> {
    Some(Direction::Left)
}

fn spot_has_snake(spot: &Coord, snakes: &Vec<Battlesnake>) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        snake_parts.push(snake.head);
        snake_parts.append(&mut snake.body.clone());
    }
    snake_parts.contains(&spot)
}

#[cfg(test)]
mod spot_has_snake_tests {
    use super::*;

    #[test]
    fn no_snakes_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 7 };
        assert_eq!(spot_has_snake(&spot, &snakes), false);
    }

    #[test]
    fn head_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![Coord { x: 3, y: 3 }, Coord { x: 3, y: 2 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 2, y: 3 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn tail_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![Coord { x: 3, y: 3 }, Coord { x: 3, y: 2 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 3, y: 2 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn hettie_is_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 0, y: 0 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn i_am_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 5 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }
}

fn spot_might_have_snake(spot: &Coord, snakes: &Vec<Battlesnake>, me: &Battlesnake) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        if snake.id != me.id && snake.length >= me.length {
            let head = snake.head;

            snake_parts.push(head.left());
            snake_parts.push(head.right());
            snake_parts.push(head.up());
            snake_parts.push(head.down());
        }
    }
    if snake_parts.contains(&spot) {
        return true;
    }

    false
}

#[cfg(test)]
mod spot_might_have_snake_tests {
    use super::*;

    #[test]
    fn no_snakes_in_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "Hettie".to_string(),
            head: Coord { x: 0, y: 0 },
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie.clone(), me.clone()];
        let spot = Coord { x: 5, y: 7 };
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), false);
    }

    #[test]
    fn larger_snake_head_right_of_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.right();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn larger_snake_head_left_of_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.left();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn same_size_snake_head_above_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.down();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn same_size_snake_head_below_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.up();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn smaller_snake_head_next_to_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 4,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.right();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), false);
    }

    #[test]
    fn i_am_next_to_spot() {
        let head = Coord { x: 3, y: 5 };
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: head,
            ..Default::default()
        };
        let snakes = vec![me.clone()];
        let spot = head.right();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), false);
    }
}

fn valid_move(spot: &Coord, board: &Board, me: &Battlesnake) -> bool {
    let board_width = board.width;
    let board_height = board.height;

    match spot {
        Coord { y: 0, .. } => false,
        Coord { x: 0, .. } => false,
        Coord { y, .. } if y == &board_width => false, // Rust is weird
        Coord { x, .. } if x == &board_height => false,
        spot if spot_has_snake(spot, &board.snakes) => false,
        spot if spot_might_have_snake(spot, &board.snakes, &me) => false,
        _ => true,
    }
}

#[cfg(test)]
mod valid_move_tests {
    use super::*;

    // Wall Tests
    #[test]
    fn head_will_not_hit_left_wall() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 0, y: 5 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_not_hit_right_wall() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 10, y: 5 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_not_hit_roof() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 10 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_not_hit_floor() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 0 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
    }

    // Collision Tests

    #[test]
    fn do_not_hit_me() {
        let me = Battlesnake {
            body: vec![Coord { x: 5, y: 4 }, Coord { x: 5, y: 5 }],
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn do_not_bite_hettie() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 3, y: 2 }, Coord { x: 4, y: 2 }],
            ..Default::default()
        };
        let board = Board {
            snakes: vec![hettie, me.clone()],
            ..Default::default()
        };
        let spot = Coord { x: 4, y: 2 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_travel() {
        let me = Battlesnake {
            body: vec![Coord { x: 5, y: 9 }, Coord { x: 5, y: 8 }],
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, true);
    }
}
