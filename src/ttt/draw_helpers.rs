use ggez::{
    graphics::{self, MeshBuilder, Text},
    nalgebra::Point2,
    Context,
};

use super::game::{Player};
use super::config::{PLAY_FIELD_POS, PLAY_FIELD_SIZE, SCREEN_SIZE, SQUARE_SIZE};

pub(crate) fn draw_field(mb: &mut MeshBuilder) {
    for i in 0..PLAY_FIELD_SIZE + 1 {
        let _ = mb.line(
            &[
                Point2::new(PLAY_FIELD_POS.0 + SQUARE_SIZE * i as f32, PLAY_FIELD_POS.1),
                Point2::new(
                    PLAY_FIELD_POS.0 + SQUARE_SIZE * i as f32,
                    PLAY_FIELD_POS.1 + SQUARE_SIZE * PLAY_FIELD_SIZE as f32,
                ),
            ],
            4.0,
            graphics::BLACK,
        );
    }
    for i in 0..PLAY_FIELD_SIZE + 1 {
        let _ = mb.line(
            &[
                Point2::new(PLAY_FIELD_POS.0, PLAY_FIELD_POS.1 + SQUARE_SIZE * i as f32),
                Point2::new(
                    PLAY_FIELD_POS.0 + SQUARE_SIZE * PLAY_FIELD_SIZE as f32,
                    PLAY_FIELD_POS.1 + SQUARE_SIZE * i as f32,
                ),
            ],
            4.0,
            graphics::BLACK,
        );
    }
}

pub(crate) fn draw_red_line(mb: &mut MeshBuilder, index_first: usize, index_second: usize) {
    let (point1_y, point1_x) = (index_first / 3, index_first % 3);
    let (point2_y, point2_x) = (index_second / 3, index_second % 3);
    let red_color = graphics::Color::from_rgb_u32(0x00FF0000);
    let _ = mb.line(
        &[
            Point2::new(
                PLAY_FIELD_POS.0 + SQUARE_SIZE * point1_x as f32 + SQUARE_SIZE / 2.0,
                PLAY_FIELD_POS.1 + SQUARE_SIZE * point1_y as f32 + SQUARE_SIZE / 2.0,
            ),
            Point2::new(
                PLAY_FIELD_POS.0 + SQUARE_SIZE * point2_x as f32 + SQUARE_SIZE / 2.0,
                PLAY_FIELD_POS.1 + SQUARE_SIZE * point2_y as f32 + SQUARE_SIZE / 2.0,
            ),
        ],
        10.0,
        red_color,
    );
}

pub(crate) fn draw_player1(mb: &mut MeshBuilder, pos_x: usize, pos_y: usize) {
    mb.circle(
        graphics::DrawMode::stroke(4.0),
        Point2::new(
            PLAY_FIELD_POS.0 + (pos_x as f32 + 0.5) * SQUARE_SIZE,
            PLAY_FIELD_POS.1 + (pos_y as f32 + 0.5) * SQUARE_SIZE,
        ),
        SQUARE_SIZE as f32 / 4.0,
        0.00001,
        graphics::BLACK,
    );
}

pub(crate) fn draw_player2(mb: &mut MeshBuilder, pos_x: usize, pos_y: usize) {
    let _ = mb.line(
        &[
            Point2::new(
                PLAY_FIELD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.25),
                PLAY_FIELD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.25),
            ),
            Point2::new(
                PLAY_FIELD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.75),
                PLAY_FIELD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.75),
            ),
        ],
        4.0,
        graphics::BLACK,
    );
    let _ = mb.line(
        &[
            Point2::new(
                PLAY_FIELD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.75),
                PLAY_FIELD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.25),
            ),
            Point2::new(
                PLAY_FIELD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.25),
                PLAY_FIELD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.75),
            ),
        ],
        4.0,
        graphics::BLACK,
    );
}

pub(crate) fn draw_player(mb: &mut MeshBuilder, player: Player, pos_x: usize, pos_y: usize) {
    match player {
        Player::Player1 => draw_player1(mb, pos_x, pos_y),
        Player::Player2 => draw_player2(mb, pos_x, pos_y),
    }
}

pub(crate) fn draw_text(ctx: &mut Context, text: &str) {
    let fps_display = Text::new(format!("Game: {}\nPress 'R' to restart", text));
    // When drawing through these calls, `DrawParam` will work as they are documented.
    let _ = graphics::draw(
        ctx,
        &fps_display,
        (Point2::new(0.0, SCREEN_SIZE.1 * 0.9), graphics::BLACK),
    );
}
