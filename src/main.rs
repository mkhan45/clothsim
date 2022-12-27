use egui_macroquad::macroquad;
use macroquad::prelude::*;
use main_state::MainState;

mod error;
mod main_state;

#[macroquad::main("Gravity")]
async fn main() -> Result<(), error::SimError> {
    next_frame().await;

    let mut main_state = MainState::default();

    loop {
        main_state.draw()?;
        main_state.update()?;
        next_frame().await;
    }
}
