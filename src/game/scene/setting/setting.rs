use macroquad::prelude::*;

use crate::game::{prefabs::button::button::Button, scene::menu};

    
fn setting_back(){
    let button_width = 200.0;
    let button_height = 50.0;

    let mut back = Button::button(
    screen_width() / 2.0 - button_width / 2.0,  // Центр по X
    screen_height() / 2.0 - 120.0 / 2.0, // Центр по Y
    button_width,
    button_height,
    "Back");
    // Обработка наведения
    if back.rect.contains(Vec2::from(mouse_position())) {
        back.color = DARKBLUE;
    } else {
        back.color = BLUE;
    }
    
    // Обработка клика
    if back.is_clicked() {
        println!("Button clicked to back!");
    }
    back.draw();
}

fn setting_sound(){
    let button_width = 200.0;
    let button_height = 50.0;
    let mut sound = Button::button(
    screen_width() / 2.0 - button_width / 2.0,  // Центр по X
    screen_height() / 2.0 - 0.0 / 2.0, // Центр по Y
    button_width,
    button_height,
    "Sound");

    // Обработка наведения
    if sound.rect.contains(Vec2::from(mouse_position())) {
        sound.color = DARKBLUE;
    } else {
        sound.color = BLUE;
    }
    
    // Обработка клика
    if sound.is_clicked() {
        println!("Button clicked to settings!");
    }




    sound.draw();
}

fn setting_graphics(){
    let button_width = 200.0;
    let button_height = 50.0;
    
    let mut graphics = Button::button(
    screen_width() / 2.0 - button_width / 2.0,  // Центр по X
    screen_height() / 2.0 + 120.0 / 2.0, // Центр по Y
    button_width,
    button_height,
    "Graphics");

    // Обработка наведения
    if graphics.rect.contains(Vec2::from(mouse_position())) {
        graphics.color = DARKBLUE;
    } else {
        graphics.color = BLUE;
    }
    
    // Обработка клика
    if graphics.is_clicked() {
        println!("Button clicked to exit!");
    }




    graphics.draw();
}

pub fn setting_start() {
    // Placeholder for game logic
     draw_text(
    "Settings",
    screen_width() / 2.0 - 120.0 / 2.0,
    screen_height() / 2.0 - 240.0 / 2.0, // Центр по Y,
    30.0,
    WHITE);

     setting_back();
     setting_sound();
     setting_graphics();


     
    
    

    
}
