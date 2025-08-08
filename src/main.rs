use game::*;
use settings::*;

mod components;
mod game;
mod physics;
mod settings;
mod systems;
mod title;
mod world;

// States
pub enum State {
    TitleScreen,
    Game(Box<Option<Game>>),
    Quit,
}

fn main() {
    // Create Raylib handle and thread
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Witch Hunt")
        .build();

    // Set FPS
    rl.set_target_fps(60);

    // Set state to title screen
    let mut current_state = State::TitleScreen;

    // Main application loop
    while !rl.window_should_close() {
        // Match current state
        match current_state {
            // State: Title Screen
            State::TitleScreen => {
                // Next state is None
                let mut next_state: Option<State> = None;

                // Update
                title::update(&mut rl, &mut next_state);

                // Render
                title::render(&mut rl, &thread, &mut next_state);

                // If next state has been set then change current state accordingly
                if let Some(state) = next_state {
                    current_state = state;
                }
            }
            // State: Game
            State::Game(ref mut new_game) => {
                // Next state is None
                let mut next_state: Option<State> = None;

                // Get game
                if let Some(game) = new_game.as_mut() {
                    // Update
                    game::update(&mut rl, &mut next_state, game);

                    // Render
                    game::render(&mut rl, &thread, game);
                }

                // If next state has been set then change current state accordingly
                if let Some(state) = next_state {
                    current_state = state;
                }
            }
            // State: Quit
            State::Quit => {
                break;
            }
        }
    }
}
