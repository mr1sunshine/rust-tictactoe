use super::draw_helpers::*;

use ggez::{
    event::{self, KeyCode, KeyMods, MouseButton},
    graphics::{self, DrawParam, MeshBuilder},
    Context, GameResult,
};

use super::game::{Cell, ChangeSelected, FieldType, Game, GameState, Player, SelectedCell};

use super::config::PLAY_FIELD_SIZE;
use super::game_logic;

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
                    let success =
                        self.set_cell_state(cell.0, cell.1, Cell::Player(Player::Player2));
                    if success {
                        game_logic::make_best_move(self);
                    }
                }
            }
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let field_type = Game::get_field_type(x, y);
        if field_type == FieldType::PlayField {
            self.select_cell(x, y);
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::R => self.clear(),
            KeyCode::Left => self.move_selected_cell(ChangeSelected::Left),
            KeyCode::Right => self.move_selected_cell(ChangeSelected::Right),
            KeyCode::Up => self.move_selected_cell(ChangeSelected::Up),
            KeyCode::Down => self.move_selected_cell(ChangeSelected::Down),
            KeyCode::Space => {
                let game_state = self.get_state();
                if game_state == GameState::InProgress {
                    let success = self.make_move_on_selected_cell(Player::Player2);
                    if success {
                        game_logic::make_best_move(self);
                    }
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, graphics::Color::from_rgb_u32(0xB0B0B0));

        let mb = &mut MeshBuilder::new();

        draw_field(mb);

        for i in 0..PLAY_FIELD_SIZE {
            for j in 0..PLAY_FIELD_SIZE {
                let cell_state = self.get_cell_state(i, j);
                if let Cell::Player(player) = cell_state {
                    draw_player(mb, player, i, j);
                }
            }
        }

        let cells = self.get_cells();
        let game_state = Game::get_game_state(&cells);
        match &game_state {
            GameState::GameWon { player: _, cells } => {
                draw_red_line(mb, cells[0], cells[2]);
            }
            _ => (),
        }

        if let SelectedCell::Selected { x, y } = self.get_selected_cell() {
            draw_selected_cell(mb, x, y);
        }

        let text = game_state_to_str(&game_state);
        draw_text(_ctx, &text);
        let mbb = mb.build(_ctx)?;
        ggez::graphics::draw(_ctx, &mbb, DrawParam::default())?;

        graphics::present(_ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }
}

fn game_state_to_str(game_state: &GameState) -> String {
    match game_state {
        GameState::Tie => String::from("Tie"),
        GameState::InProgress => String::from("In progress"),
        GameState::GameWon { player, .. } => match player {
            Player::Player1 => String::from("Computer won"),
            Player::Player2 => String::from("Player won"),
        },
    }
}
