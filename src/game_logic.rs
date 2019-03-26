#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Empty,
    Player,
    Computer,
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum GameResult {
    NoWinner,
    Winner {
        winner_type: CellState,
        elements: Vec<(usize, usize)>,
    },
}

impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

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
pub fn get_game_result(elements: &[CellState]) -> GameResult {
    for i in 0..3 {
        if elements[i * 3] != CellState::Empty
            && elements[i * 3] == elements[i * 3 + 1]
            && elements[i * 3 + 1] == elements[i * 3 + 2]
        {
            return GameResult::Winner {
                winner_type: elements[i * 3],
                elements: [(i, 0), (i, 1), (i, 2)].to_vec(),
            };
        }
    }
    for i in 0..3 {
        if elements[i] != CellState::Empty
            && elements[i] == elements[i + 3]
            && elements[i + 3] == elements[i + 6]
        {
            return GameResult::Winner {
                winner_type: elements[i],
                elements: [(0, i), (1, i), (2, i)].to_vec(),
            };
        }
    }

    if elements[0] != CellState::Empty && elements[0] == elements[4] && elements[4] == elements[8] {
        return GameResult::Winner {
            winner_type: elements[0],
            elements: [(0, 0), (1, 1), (2, 2)].to_vec(),
        };
    }

    if elements[2] != CellState::Empty && elements[2] == elements[4] && elements[4] == elements[6] {
        return GameResult::Winner {
            winner_type: elements[2],
            elements: [(0, 2), (1, 1), (2, 0)].to_vec(),
        };
    }

    GameResult::NoWinner
}

pub fn is_game_finished(elements: &[CellState]) -> bool {
    for cell_state in elements {
        match *cell_state {
            CellState::Empty => {
                return false;
            }
            _ => (),
        }
    }
    true
}

fn minimax1(elements: &mut [CellState], is_max: bool) -> i32 {
    let is_game_finished = is_game_finished(elements);
    if is_game_finished {
        let board_result = get_game_result(elements);
        match board_result {
            GameResult::NoWinner => return 0,
            GameResult::Winner {
                winner_type,
                elements: _,
            } => {
                if winner_type == CellState::Computer {
                    return 10;
                } else {
                    return -10;
                }
            }
        }
    }

    let mut best = 0;

    if is_max {
        best = -1000;
        for i in 0..elements.len() {
            if elements[i] == CellState::Empty {
                elements[i] = CellState::Player;

                best = std::cmp::max(best, minimax1(elements, !is_max));

                elements[i] = CellState::Empty;
            }
        }
    } else {
        best = 1000;
        for i in 0..elements.len() {
            if elements[i] == CellState::Empty {
                elements[i] = CellState::Computer;

                best = std::cmp::min(best, minimax1(elements, !is_max));

                elements[i] = CellState::Empty;
            }
        }
    }
    best
}

// fn minimax(elements: &mut [CellState], is_max: bool) -> i32 {
//     let mut best: i32;
//     let game_result = get_game_result(elements);
//     match game_result {
//         GameResult::NoWinner => {
//             if is_game_finished(elements) {
//                 return 0;
//             }
//             if is_max {
//                 best = -1000;
//                 for i in 0..elements.len() {
//                     if elements[i] == CellState::Empty {
//                         elements[i] = CellState::Player;

//                         best = std::cmp::max(best, minimax(elements, !is_max));

//                         elements[i] = CellState::Empty;
//                     }
//                 }
//             } else {
//                 best = 1000;
//                 for i in 0..elements.len() {
//                     if elements[i] == CellState::Empty {
//                         elements[i] = CellState::Computer;

//                         best = std::cmp::min(best, minimax(elements, !is_max));

//                         elements[i] = CellState::Empty;
//                     }
//                 }
//             }
//         }
//         GameResult::Winner {
//             winner_type,
//             elements: _,
//         } => {
//             if winner_type == CellState::Computer {
//                 return 1;
//             } else {
//                 return -1;
//             }
//         }
//     }

//     best
// }

pub fn find_best_move(elements: &mut [CellState]) -> Option<usize> {
    let mut best_move: usize = 0;
    let mut best_val = std::i32::MIN;
    let mut best_move_found = false;

    println!("====================");
    for i in 0..elements.len() {
        println!("state of {0} is {1}", i, elements[i]);
        if elements[i] == CellState::Empty {
            elements[i] = CellState::Computer;
            let val = minimax1(elements, true);
            println!("best move val for {0} is {1}", i, val);
            elements[i] = CellState::Empty;

            if val > best_val {
                best_move_found = true;
                best_val = val;
                best_move = i;
            }
        }
    }

    if best_move_found {
        Some(best_move)
    } else {
        None
    }
}

pub fn make_best_move(elements: &mut [Cell]) {
    let best_move = get_best_move(elements);
    elements[best_move] = Cell::Player(Player::Player1);
    // match best_move {
    //     Some(best_move) => elements[best_move] = CellState::Computer,
    //     _ => (),
    // }
}
