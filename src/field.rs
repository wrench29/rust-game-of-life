use raylib::{color::Color, drawing::RaylibDraw, RaylibHandle, RaylibThread};

use crate::game::{GameCellColor, GameCellCombined};

pub struct GameField {
    rl: RaylibHandle,
    thread: RaylibThread,
    cell_size: usize,
}

impl GameField {
    pub fn new(cells_count: usize) -> Self {
        let window_size = 800;
        let cell_size = window_size / cells_count;
        let window_size = cell_size * cells_count;
        let (rl, thread) = raylib::init()
            .size(window_size as i32, window_size as i32)
            .title("Game of Life")
            .vsync()
            .build();
        Self {
            rl,
            thread,
            cell_size,
        }
    }
    pub fn draw_cells(
        &mut self,
        cells: &Vec<GameCellCombined>,
        generation: usize,
        generations_per_second: usize,
    ) {
        let mut draw_handle = self.rl.begin_drawing(&self.thread);
        draw_handle.clear_background(Color::WHITE);
        for cell in cells {
            let x_absolute = self.cell_size * cell.position.x;
            let y_absolute = self.cell_size * cell.position.y;
            draw_handle.draw_rectangle(
                x_absolute as i32,
                y_absolute as i32,
                self.cell_size as i32,
                self.cell_size as i32,
                Self::color_to_rl_color(cell.cell.color),
            );
        }
        let generation_text = format!(
            "Generation: {}. Population speed: {} gen/s. Use arrows to change.",
            generation, generations_per_second
        );
        draw_handle.draw_text(&generation_text, 10, 10, 20, Color::SILVER);
    }
    pub fn should_close(&self) -> bool {
        self.rl.window_should_close()
    }
    pub fn is_up_pressed(&self) -> bool {
        use raylib::consts::KeyboardKey::*;
        self.rl.is_key_released(KEY_UP)
    }
    pub fn is_down_pressed(&self) -> bool {
        use raylib::consts::KeyboardKey::*;
        self.rl.is_key_released(KEY_DOWN)
    }
    fn color_to_rl_color(color: GameCellColor) -> Color {
        match color {
            GameCellColor::None => Color::BLACK,
            GameCellColor::Red => Color::RED,
            GameCellColor::Green => Color::GREEN,
            GameCellColor::Blue => Color::BLUE,
            GameCellColor::Orange => Color::ORANGE,
        }
    }
}
