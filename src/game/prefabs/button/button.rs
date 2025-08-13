use macroquad::prelude::*;
pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub color: Color,
}

impl Button {


    pub fn button(x: f32, y: f32, width: f32, height: f32, text: &str) -> Self {
        Button {
            rect: Rect::new(x, y, width, height),
            text: text.to_string(),
            color: BLUE,
        }
    }

    pub fn draw(&self) {
        // Рисуем прямоугольник кнопки
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.color);
        
        // Центрируем текст внутри кнопки
        let text_size = measure_text(&self.text, None, 20, 1.0);
        let text_x = self.rect.x + (self.rect.w - text_size.width) / 2.0;
        let text_y = self.rect.y + (self.rect.h - text_size.height) / 2.0 + text_size.offset_y;
        
        draw_text(&self.text, text_x, text_y, 20.0, WHITE);
    }

    pub fn is_clicked(&self) -> bool {
        // Проверяем нажатие мыши и пересечение с областью кнопки
        is_mouse_button_pressed(MouseButton::Left) && 
        self.rect.contains(Vec2::from(mouse_position()))
    }
}