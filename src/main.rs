use macroquad::prelude::*;
use rayon::join;
use crate::game::prefabs::button::button::Button;

mod game;

#[macroquad::main("Pong Game")]
async fn main() {
    loop {
        clear_background( BLACK);

        game::logic::logic_function(); // Call the game logic function

        



        next_frame().await;
        
    }
    
}
