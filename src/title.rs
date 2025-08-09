use crate::game::*;
use crate::State;
use raylib::prelude::*;

// Update
pub fn update(rl: &mut RaylibHandle, next_state: &mut Option<State>) {
    // If cursor is hidden then enable it
    if rl.is_cursor_hidden() {
        rl.enable_cursor();
    }

    // Set text style for GUI
    rl.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 30);

    // Press Enter or Space to continue
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        // Set state to Game and create new game
        match Game::new() {
            Ok(game) => *next_state = Some(State::Game(Box::new(game))),
            Err(e) => {
                eprintln!("Failed to create game: {e}");
                *next_state = Some(State::Quit);
            }
        }
    }
}

// Render
pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, next_state: &mut Option<State>) {
    // Begin drawing frame
    let mut d = rl.begin_drawing(thread);

    // Clear frame
    d.clear_background(Color::WHITE);

    // Start button
    if d.gui_button(Rectangle::new(300.0, 150.0, 200.0, 50.0), "START") {
        // Set state to Game and create new game
        match Game::new() {
            Ok(game) => *next_state = Some(State::Game(Box::new(game))),
            Err(e) => {
                eprintln!("Failed to create game: {e}");
                *next_state = Some(State::Quit);
            }
        }
    }

    // Quit button
    if d.gui_button(Rectangle::new(300.0, 250.0, 200.0, 50.0), "QUIT") {
        *next_state = Some(State::Quit);
    }
}
