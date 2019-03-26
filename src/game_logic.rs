#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum GameState {
    GameWon { player: Player, cells: Vec<usize> },
    Tie,
    InProgress,
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Player(Player),
}

pub fn get_game_state(elements: &[Cell]) -> GameState {
    for i in 0..3 {
        if elements[i * 3] != Cell::Empty
            && elements[i * 3] == elements[i * 3 + 1]
            && elements[i * 3 + 1] == elements[i * 3 + 2]
        {
            if let Cell::Player(player) = elements[i * 3] {
                return GameState::GameWon {
                    player: player,
                    cells: vec![i * 3, i * 3 + 1, i * 3 + 2],
                };
            }
        }
    }
    for i in 0..3 {
        if elements[i] != Cell::Empty
            && elements[i] == elements[i + 3]
            && elements[i + 3] == elements[i + 6]
        {
            if let Cell::Player(player) = elements[i] {
                return GameState::GameWon {
                    player: player,
                    cells: vec![i, i + 3, i + 6],
                };
            }
        }
    }
    if elements[0] != Cell::Empty && elements[0] == elements[4] && elements[4] == elements[8] {
        if let Cell::Player(player) = elements[0] {
            return GameState::GameWon {
                player: player,
                cells: vec![0, 4, 8],
            };
        }
    }

    if elements[2] != Cell::Empty && elements[2] == elements[4] && elements[4] == elements[6] {
        if let Cell::Player(player) = elements[2] {
            return GameState::GameWon {
                player: player,
                cells: vec![2, 4, 6],
            };
        }
    }

    for i in 0..elements.len() {
        if elements[i] == Cell::Empty {
            return GameState::InProgress;
        }
    }

    GameState::Tie
}

fn max_search(elements: &mut [Cell]) -> i32 {
    let game_state = get_game_state(elements);
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
    let game_state = get_game_state(elements);
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

pub fn get_best_move(elements: &mut [Cell]) -> usize {
    let mut best_score = std::i32::MIN;
    let mut best_move: usize = 0;

    for i in 0..elements.len() {
        if elements[i] == Cell::Empty {
            elements[i] = Cell::Player(Player::Player1);
            let tmp_score = min_search(elements);
            elements[i] = Cell::Empty;

            if tmp_score > best_score {
                best_score = tmp_score;
                best_move = i;
            }
        }
    }

    best_move
}

pub fn make_best_move(elements: &mut [Cell]) {
    let best_move = get_best_move(elements);
    elements[best_move] = Cell::Player(Player::Player1);
    // match best_move {
    //     Some(best_move) => elements[best_move] = CellState::Computer,
    //     _ => (),
    // }
}
