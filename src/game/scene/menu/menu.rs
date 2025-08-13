use macroquad::prelude::*;

use crate::game::{prefabs::button::button::Button, scene::menu};
use crate::game::scene::setting::setting::{self, setting_start};
    
fn menu_button_paly(){
    let button_width = 200.0;
    let button_height = 50.0;

    let mut play = Button::button(
    screen_width() / 2.0 - button_width / 2.0,  // Центр по X
    screen_height() / 2.0 - 120.0 / 2.0, // Центр по Y
    button_width,
    button_height,
    "Play");
    // Обработка наведения
    if play.rect.contains(Vec2::from(mouse_position())) {
        play.color = DARKBLUE;
    } else {
        play.color = BLUE;
    }
    
    // Обработка клика
    if play.is_clicked() {
        println!("Button clicked to play!");
    }
    play.draw();
}

fn menu_button_setting(){
    let button_width = 200.0;
    let button_height = 50.0;
    let mut setting = Button::button(
    screen_width() / 2.0 - button_width / 2.0,  // Центр по X
    screen_height() / 2.0 - 0.0 / 2.0, // Центр по Y
    button_width,
    button_height,
    "Settings");

    // Обработка наведения
    if setting.rect.contains(Vec2::from(mouse_position())) {
        setting.color = DARKBLUE;
    } else {
        setting.color = BLUE;
    }
    
    // Обработка клика
    if setting.is_clicked() {
        println!("Button clicked to settings!");
        setting_start();
    }




    setting.draw();
}

fn menu_button_exit(){
    let button_width = 200.0;
    let button_height = 50.0;
    
    let mut exit = Button::button(
    screen_width() / 2.0 - button_width / 2.0,  // Центр по X
    screen_height() / 2.0 + 120.0 / 2.0, // Центр по Y
    button_width,
    button_height,
    "Exit");

    // Обработка наведения
    if exit.rect.contains(Vec2::from(mouse_position())) {
        exit.color = DARKBLUE;
    } else {
        exit.color = BLUE;
    }
    
    // Обработка клика
    if exit.is_clicked() {
        println!("Button clicked to exit!");
    }




    exit.draw();
}

pub fn menu_start() {
    // Placeholder for game logic
     draw_text(
    "Pong game",
    screen_width() / 2.0 - 120.0 / 2.0,
    screen_height() / 2.0 - 240.0 / 2.0, // Центр по Y,
    30.0,
    WHITE);

     menu_button_paly();
     menu_button_setting();
     menu_button_exit();


     
    
    

    
}
