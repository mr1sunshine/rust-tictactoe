use ggez::event;

use ggez::GameResult;

mod ttt;

use ttt::game::Game;

use ttt::config::{AUTHOR, GAME_NAME, SCREEN_SIZE};

fn main() -> GameResult {
    // Make a Context.
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new(GAME_NAME, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(GAME_NAME))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut Game::new();
    event::run(ctx, event_loop, state)
}
