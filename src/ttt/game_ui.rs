use ggez::{
    event, event::MouseButton, graphics, graphics::DrawParam, nalgebra::Point2, Context, GameResult,
};

use super::game::{
    Cell, FieldType, Game, GameState, Player, PLAY_FIELD_POS, PLAY_FIELD_SIZE, SQUARE_SIZE,
};
use super::game_logic;

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let field_type = Game::get_field_type(x, y);
            if field_type == FieldType::PlayField {
                let cells = self.get_cells();
                let game_state = Game::get_game_state(&cells);
                if game_state == GameState::InProgress {
                    let cell = Game::get_cell(x, y);
                    self.set_cell_state(cell.0, cell.1, Cell::Player(Player::Player2));
                    game_logic::make_best_move(self);
                }
            }
        }
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        // println!("draw called!");
        // Draw code here...
        graphics::clear(_ctx, graphics::Color::from_rgb_u32(0xB0B0B0));

        let mb = &mut graphics::MeshBuilder::new();
        for i in 0..PLAY_FIELD_SIZE + 1 {
            mb.line(
                &[
                    Point2::new(PLAY_FIELD_POS.0 + SQUARE_SIZE * i as f32, PLAY_FIELD_POS.1),
                    Point2::new(
                        PLAY_FIELD_POS.0 + SQUARE_SIZE * i as f32,
                        PLAY_FIELD_POS.1 + SQUARE_SIZE * PLAY_FIELD_SIZE as f32,
                    ),
                ],
                4.0,
                graphics::BLACK,
            )?;
        }
        for i in 0..PLAY_FIELD_SIZE + 1 {
            mb.line(
                &[
                    Point2::new(PLAY_FIELD_POS.0, PLAY_FIELD_POS.1 + SQUARE_SIZE * i as f32),
                    Point2::new(
                        PLAY_FIELD_POS.0 + SQUARE_SIZE * PLAY_FIELD_SIZE as f32,
                        PLAY_FIELD_POS.1 + SQUARE_SIZE * i as f32,
                    ),
                ],
                4.0,
                graphics::BLACK,
            )?;
        }
        for i in 0..PLAY_FIELD_SIZE {
            for j in 0..PLAY_FIELD_SIZE {
                let cell_state = self.get_cell_state(i as usize, j as usize);
                if let Cell::Player(player) = cell_state {
                    match player {
                        Player::Player1 => {
                            mb.circle(
                                graphics::DrawMode::stroke(4.0),
                                Point2::new(
                                    PLAY_FIELD_POS.0 + (i as f32 + 0.5) * SQUARE_SIZE,
                                    PLAY_FIELD_POS.1 + (j as f32 + 0.5) * SQUARE_SIZE,
                                ),
                                SQUARE_SIZE as f32 / 4.0,
                                0.00001,
                                graphics::BLACK,
                            );
                        }
                        Player::Player2 => {
                            mb.line(
                                &[
                                    Point2::new(
                                        PLAY_FIELD_POS.0 + SQUARE_SIZE * (i as f32 + 0.25),
                                        PLAY_FIELD_POS.1 + SQUARE_SIZE * (j as f32 + 0.25),
                                    ),
                                    Point2::new(
                                        PLAY_FIELD_POS.0 + SQUARE_SIZE * (i as f32 + 0.75),
                                        PLAY_FIELD_POS.1 + SQUARE_SIZE * (j as f32 + 0.75),
                                    ),
                                ],
                                4.0,
                                graphics::BLACK,
                            )?;
                            mb.line(
                                &[
                                    Point2::new(
                                        PLAY_FIELD_POS.0 + SQUARE_SIZE * (i as f32 + 0.75),
                                        PLAY_FIELD_POS.1 + SQUARE_SIZE * (j as f32 + 0.25),
                                    ),
                                    Point2::new(
                                        PLAY_FIELD_POS.0 + SQUARE_SIZE * (i as f32 + 0.25),
                                        PLAY_FIELD_POS.1 + SQUARE_SIZE * (j as f32 + 0.75),
                                    ),
                                ],
                                4.0,
                                graphics::BLACK,
                            )?;
                        }
                    }
                }
            }
        }

        let cells = self.get_cells();
        let game_state = Game::get_game_state(&cells);
        match game_state {
            GameState::GameWon { player: _, cells } => {
                let (point1_y, point1_x) = (cells[0] / 3, cells[0] % 3);
                let (point2_y, point2_x) = (cells[2] / 3, cells[2] % 3);
                let red_color = graphics::Color::from_rgb_u32(0x00FF0000);
                mb.line(
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
                )?;
            }
            _ => (),
        }
        // ggez::graphics::rectangle(_ctx, graphics::DrawMode::Fill, border_rect);
        let mbb = mb.build(_ctx)?;
        ggez::graphics::draw(_ctx, &mbb, DrawParam::default())?;

        graphics::present(_ctx)?;
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        Ok(())
    }
}
