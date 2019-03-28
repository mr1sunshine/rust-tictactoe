use ggez::event;

use ggez::GameResult;

mod ttt;

use ttt::game::{Game, SCREEN_SIZE};

fn main() -> GameResult {
    // Make a Context.
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("helloworld", "ggez")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("TicTacToe"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let state = &mut Game::new();
    event::run(ctx, event_loop, state)
}
