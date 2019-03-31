use super::config::{PLAY_FIELD_POS, PLAY_FIELD_SIZE, SQUARE_SIZE};

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

#[derive(Debug, PartialEq)]
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

pub struct Game {
    cell_states: Vec<Cell>,
    selected_cell: SelectedCell,
}

#[derive(PartialEq)]
pub enum FieldType {
    PlayField,
    OutField,
}

#[derive(PartialEq, Clone)]
pub enum SelectedCell {
    NotSelected,
    Selected { x: usize, y: usize },
}

pub enum ChangeSelected {
    Left,
    Up,
    Right,
    Down,
}

impl Game {
    pub(crate) fn new() -> Self {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        Game {
            cell_states: vec![Cell::Empty; PLAY_FIELD_SIZE * PLAY_FIELD_SIZE],
            selected_cell: SelectedCell::NotSelected,
        }
    }

    pub(crate) fn clear(&mut self) {
        *self = Game::new();
        // self.cell_states = vec![Cell::Empty; PLAY_FIELD_SIZE * PLAY_FIELD_SIZE];
        // self.S
    }

    pub(crate) fn get_field_type(x: f32, y: f32) -> FieldType {
        if PLAY_FIELD_POS.0 < x
            && x < PLAY_FIELD_POS.0 + SQUARE_SIZE * PLAY_FIELD_SIZE as f32
            && PLAY_FIELD_POS.1 < y
            && y < PLAY_FIELD_POS.1 + SQUARE_SIZE * PLAY_FIELD_SIZE as f32
        {
            return FieldType::PlayField;
        }

        FieldType::OutField
    }

    pub(crate) fn get_selected_cell(&self) -> SelectedCell {
        self.selected_cell.clone()
    }

    pub(crate) fn get_state(&self) -> GameState {
        Game::get_game_state(&self.cell_states)
    }

    pub(crate) fn get_cell(x: f32, y: f32) -> (usize, usize) {
        let cell_x = (x - PLAY_FIELD_POS.0) / SQUARE_SIZE;
        let cell_y = (y - PLAY_FIELD_POS.1) / SQUARE_SIZE;

        (cell_x as usize, cell_y as usize)
    }

    pub(crate) fn select_cell(&mut self, x: f32, y: f32) {
        let (index_x, index_y) = Game::get_cell(x, y);
        self.selected_cell = SelectedCell::Selected {
            x: index_x,
            y: index_y,
        };
    }

    pub(crate) fn get_cell_state(&self, row: usize, column: usize) -> Cell {
        self.cell_states[row + column * PLAY_FIELD_SIZE as usize]
    }

    pub(crate) fn set_cell_state(&mut self, row: usize, column: usize, cell_state: Cell) -> bool {
        if self.cell_states[row + column * PLAY_FIELD_SIZE as usize] == Cell::Empty {
            self.cell_states[row + column * PLAY_FIELD_SIZE as usize] = cell_state;
            return true;
        }
        false
    }

    pub(crate) fn move_selected_cell(&mut self, direction: ChangeSelected) {
        match self.get_state() {
            GameState::GameWon { .. } => return,
            _ => (),
        }
        if self.selected_cell == SelectedCell::NotSelected {
            self.selected_cell = SelectedCell::Selected { x: 0, y: 0 };
            return;
        }
        match direction {
            ChangeSelected::Down => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if y < PLAY_FIELD_SIZE - 1 {
                        self.selected_cell = SelectedCell::Selected { x: x, y: y + 1 };
                    }
                }
            }
            ChangeSelected::Up => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if y > 0 {
                        self.selected_cell = SelectedCell::Selected { x: x, y: y - 1 };
                    }
                }
            }
            ChangeSelected::Right => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if x < PLAY_FIELD_SIZE - 1 {
                        self.selected_cell = SelectedCell::Selected { x: x + 1, y: y };
                    }
                }
            }
            ChangeSelected::Left => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if x > 0 {
                        self.selected_cell = SelectedCell::Selected { x: x - 1, y: y };
                    }
                }
            }
        }
    }

    pub(crate) fn get_game_state(cell_states: &[Cell]) -> GameState {
        for i in 0..3 {
            if cell_states[i * 3] != Cell::Empty
                && cell_states[i * 3] == cell_states[i * 3 + 1]
                && cell_states[i * 3 + 1] == cell_states[i * 3 + 2]
            {
                if let Cell::Player(player) = cell_states[i * 3] {
                    return GameState::GameWon {
                        player: player,
                        cells: vec![i * 3, i * 3 + 1, i * 3 + 2],
                    };
                }
            }
        }
        for i in 0..3 {
            if cell_states[i] != Cell::Empty
                && cell_states[i] == cell_states[i + 3]
                && cell_states[i + 3] == cell_states[i + 6]
            {
                if let Cell::Player(player) = cell_states[i] {
                    return GameState::GameWon {
                        player: player,
                        cells: vec![i, i + 3, i + 6],
                    };
                }
            }
        }
        if cell_states[0] != Cell::Empty
            && cell_states[0] == cell_states[4]
            && cell_states[4] == cell_states[8]
        {
            if let Cell::Player(player) = cell_states[0] {
                return GameState::GameWon {
                    player: player,
                    cells: vec![0, 4, 8],
                };
            }
        }

        if cell_states[2] != Cell::Empty
            && cell_states[2] == cell_states[4]
            && cell_states[4] == cell_states[6]
        {
            if let Cell::Player(player) = cell_states[2] {
                return GameState::GameWon {
                    player: player,
                    cells: vec![2, 4, 6],
                };
            }
        }

        for i in 0..cell_states.len() {
            if cell_states[i] == Cell::Empty {
                return GameState::InProgress;
            }
        }

        GameState::Tie
    }

    pub(crate) fn get_cells(&self) -> Vec<Cell> {
        self.cell_states.clone()
    }

    pub(crate) fn make_move(&mut self, cell_id: usize, player: Player) {
        let game_state = Game::get_game_state(&self.cell_states);
        if game_state == GameState::InProgress {
            if cell_id < self.cell_states.len() {
                self.cell_states[cell_id] = Cell::Player(player);
            } else {
                unreachable!();
            }
        }
    }

    pub(crate) fn make_move_on_selected_cell(&mut self, player: Player) -> bool {
        match self.selected_cell {
            SelectedCell::NotSelected => return false,
            SelectedCell::Selected { x, y } => {
                let index = x + PLAY_FIELD_SIZE * y;
                match self.cell_states[index] {
                    Cell::Empty => {
                        self.cell_states[index] = Cell::Player(player);
                        return true;
                    }
                    _ => return false,
                }
            }
        }
    }
}
