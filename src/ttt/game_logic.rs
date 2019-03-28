use super::game::{Cell, Game, GameState, Player};

fn max_search(elements: &mut [Cell]) -> i32 {
    let game_state = Game::get_game_state(elements);
    match game_state {
        GameState::GameWon { player, .. } => {
            return if player == Player::Player1 { 10 } else { -10 };
        }
        GameState::Tie => return 0,
        GameState::InProgress => (),
    }

    let mut score = std::i32::MIN;

    for i in 0..elements.len() {
        if elements[i] == Cell::Empty {
            elements[i] = Cell::Player(Player::Player1);
            score = std::cmp::max(score, min_search(elements));
            elements[i] = Cell::Empty;
        }
    }
    score
}

fn min_search(elements: &mut [Cell]) -> i32 {
    let game_state = Game::get_game_state(elements);
    match game_state {
        GameState::GameWon { player, .. } => {
            return if player == Player::Player1 { 10 } else { -10 };
        }
        GameState::Tie => return 0,
        GameState::InProgress => (),
    }

    let mut score = std::i32::MAX;

    for i in 0..elements.len() {
        if elements[i] == Cell::Empty {
            elements[i] = Cell::Player(Player::Player2);
            score = std::cmp::min(score, max_search(elements));
            elements[i] = Cell::Empty;
        }
    }
    score
}

pub fn get_best_move(elements: &mut [Cell]) -> Option<usize> {
    let mut best_score = std::i32::MIN;
    let mut best_move: usize = 0;
    let mut best_move_found = false;

    for i in 0..elements.len() {
        if elements[i] == Cell::Empty {
            elements[i] = Cell::Player(Player::Player1);
            let tmp_score = min_search(elements);
            elements[i] = Cell::Empty;

            if tmp_score > best_score {
                best_score = tmp_score;
                best_move = i;
                best_move_found = true;
            }
        }
    }

    if best_move_found {
        Some(best_move)
    } else {
        None
    }
}

pub fn make_best_move(game: &mut Game) {
    let mut cells = game.get_cells();
    let best_move = get_best_move(&mut cells);
    if let Some(id) = best_move {
        game.make_move(id, Player::Player1);
    }
}
