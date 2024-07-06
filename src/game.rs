use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct GameCellPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameCellColor {
    None,
    Red,
    Green,
    Blue,
    Orange,
}

#[derive(Clone, Copy)]
pub struct GameCell {
    pub is_alive: bool,
    pub color: GameCellColor,
}

pub struct GameCellCombined {
    pub cell: GameCell,
    pub position: GameCellPosition,
}

pub struct GameLogic {
    field_size: usize,
    field: Vec<Vec<GameCell>>,
    last_update_time_millis: u128,
    update_speed: usize,
}

impl Default for GameCell {
    fn default() -> Self {
        Self {
            is_alive: false,
            color: GameCellColor::None,
        }
    }
}

impl GameLogic {
    pub fn new(field_size: usize) -> Self {
        let mut field = vec![vec![GameCell::default(); field_size]; field_size];
        Self::spawn_first_generation(&mut field, field_size);
        Self {
            field_size,
            field,
            last_update_time_millis: 0,
            update_speed: 1000,
        }
    }
    pub fn get_cells(&self) -> Vec<GameCellCombined> {
        let mut cells: Vec<GameCellCombined> = vec![];
        for y in 0..self.field_size {
            for x in 0..self.field_size {
                let cell = self.field[y][x];
                if cell.is_alive {
                    cells.push(GameCellCombined {
                        cell,
                        position: GameCellPosition { x, y },
                    });
                }
            }
        }
        cells
    }
    pub fn update_cells(&mut self) -> bool {
        if !self.check_and_update_time() {
            return false;
        }
        let mut field_copy = self.field.clone();
        for y in 0..self.field_size {
            for x in 0..self.field_size {
                field_copy[y][x] = self.analyze_cell(x, y);
            }
        }
        self.field = field_copy;
        true
    }
    pub fn set_update_speed(&mut self, generation_per_second: usize) {
        self.update_speed = 1000 / generation_per_second;
    }
    fn check_and_update_time(&mut self) -> bool {
        let start_time = SystemTime::now();
        let timestamp = start_time.duration_since(UNIX_EPOCH).unwrap();
        let current_time = timestamp.as_millis();
        let time_difference = current_time - self.last_update_time_millis;
        if time_difference > self.update_speed as u128 {
            self.last_update_time_millis = current_time;
            return true;
        }
        false
    }
    fn analyze_cell(&self, x: usize, y: usize) -> GameCell {
        let mut alive_cells_count = 0;
        let mut red_cells = 0;
        let mut green_cells = 0;
        let mut blue_cells = 0;
        let mut orange_cells = 0;
        for y_rel in -1..=1 {
            for x_rel in -1..=1 {
                if y_rel == 0 && x_rel == 0 {
                    continue;
                }
                let y_abs = y as i32 + y_rel;
                let x_abs = x as i32 + x_rel;
                let (x_abs, y_abs) = Self::make_coords_absolute(self.field_size, x_abs, y_abs);
                if self.field[y_abs][x_abs].is_alive {
                    alive_cells_count += 1;
                    match &self.field[y_abs][x_abs].color {
                        GameCellColor::Red => red_cells += 1,
                        GameCellColor::Green => green_cells += 1,
                        GameCellColor::Blue => blue_cells += 1,
                        GameCellColor::Orange => orange_cells += 1,
                        _ => {}
                    }
                }
            }
        }
        let mut new_cell = GameCell::default();
        if self.field[y][x].is_alive {
            if (2..=3).contains(&alive_cells_count) {
                new_cell.is_alive = true;
                new_cell.color = self.field[y][x].color;
            }
        } else if alive_cells_count == 3 {
            new_cell.is_alive = true;
            let most_colors =
                Self::get_most_colors(red_cells, green_cells, blue_cells, orange_cells);
            new_cell.color = most_colors[rand::random::<usize>() % most_colors.len()];
        }
        new_cell
    }
    fn get_most_colors(
        red_cells: i32,
        green_cells: i32,
        blue_cells: i32,
        orange_cells: i32,
    ) -> Vec<GameCellColor> {
        let mut cells: HashMap<GameCellColor, i32> = HashMap::new();
        cells.insert(GameCellColor::Red, red_cells);
        cells.insert(GameCellColor::Green, green_cells);
        cells.insert(GameCellColor::Blue, blue_cells);
        cells.insert(GameCellColor::Orange, orange_cells);
        let colors = vec![
            GameCellColor::Red,
            GameCellColor::Green,
            GameCellColor::Blue,
            GameCellColor::Orange,
        ];
        let mut most_popular_color = GameCellColor::Red;
        for c in &colors {
            if cells.get(c) >= cells.get(&most_popular_color) {
                most_popular_color = c.clone();
            }
        }
        let mut popular_colors = vec![most_popular_color];
        for c in &colors {
            if c == &most_popular_color {
                continue;
            }
            if cells.get(c) == cells.get(&most_popular_color) {
                popular_colors.push(most_popular_color);
            }
        }
        popular_colors
    }
    fn make_coords_absolute(field_size: usize, x: i32, y: i32) -> (usize, usize) {
        let mut x_abs = x;
        let mut y_abs = y;
        let field_size = field_size as i32;
        if y_abs < 0 {
            y_abs += field_size;
        }
        if x_abs < 0 {
            x_abs += field_size;
        }
        if y_abs > (field_size - 1) {
            y_abs -= field_size;
        }
        if x_abs > (field_size - 1) {
            x_abs -= field_size;
        }
        (x_abs as usize, y_abs as usize)
    }
    fn spawn_first_generation(field: &mut Vec<Vec<GameCell>>, field_size: usize) {
        let count_of_lifes = 4;
        let colors = vec![
            GameCellColor::Red,
            GameCellColor::Green,
            GameCellColor::Blue,
            GameCellColor::Orange,
        ];
        for i in 0..count_of_lifes {
            let y_sel = rand::random::<usize>() % field_size;
            let x_sel = rand::random::<usize>() % field_size;
            for y_rel in -1..=1 {
                for x_rel in -1..=1 {
                    let do_skip = (y_rel == -1 && x_rel == -1)
                        || (y_rel == -1 && x_rel == 1)
                        || (y_rel == 1 && x_rel == -1)
                        || (y_rel == 0 && x_rel == 1);
                    if do_skip {
                        continue;
                    }
                    let y_abs = y_sel as i32 + y_rel;
                    let x_abs = x_sel as i32 + x_rel;
                    let (x_abs, y_abs) = Self::make_coords_absolute(field_size, x_abs, y_abs);
                    field[y_abs][x_abs].is_alive = true;
                    field[y_abs][x_abs].color = colors[i];
                }
            }
        }
    }
}
