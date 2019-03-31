pub static GAME_NAME: &str = "TicTacToe";
pub static AUTHOR: &str = "Alexander Ovchinnikov";

pub static SCREEN_SIZE: (f32, f32) = (960.0, 640.0);

pub static SQUARE_SIZE: f32 = 150.0;

pub static PLAY_FIELD_SIZE: usize = 3;

pub static PLAY_FIELD_POS: (f32, f32) = (
    SCREEN_SIZE.0 / 2.0 - SQUARE_SIZE * (PLAY_FIELD_SIZE as f32 / 2.0),
    SCREEN_SIZE.1 / 2.0 - SQUARE_SIZE * (PLAY_FIELD_SIZE as f32 / 2.0),
);
