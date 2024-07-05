use std::time::{SystemTime, UNIX_EPOCH};

pub struct GameLogic {
    field_size: usize,
    field: Vec<Vec<bool>>,
    last_update_time_millis: u128,
    update_speed: usize,
}

pub struct GameCell {
    pub x: usize,
    pub y: usize,
}

impl GameLogic {
    pub fn new(field_size: usize) -> Self {
        let mut field = vec![vec![false; field_size]; field_size];
        Self::spawn_first_generation(&mut field, field_size);
        Self {
            field_size,
            field,
            last_update_time_millis: 0,
            update_speed: 1000,
        }
    }
    pub fn get_cells(&self) -> Vec<GameCell> {
        let mut cells: Vec<GameCell> = vec![];
        for y in 0..self.field_size {
            for x in 0..self.field_size {
                if self.field[y][x] {
                    cells.push(GameCell { x, y });
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
    fn analyze_cell(&self, x: usize, y: usize) -> bool {
        let mut alive_cells_count = 0;
        for y_rel in -1..=1 {
            for x_rel in -1..=1 {
                if y_rel == 0 && x_rel == 0 {
                    continue;
                }
                let y_abs = y as i32 + y_rel;
                let x_abs = x as i32 + x_rel;
                let (x_abs, y_abs) = Self::make_coords_absolute(self.field_size, x_abs, y_abs);
                if self.field[y_abs][x_abs] {
                    alive_cells_count += 1;
                }
            }
        }
        let mut will_be_alive = false;
        if self.field[y][x] {
            if (2..=3).contains(&alive_cells_count) {
                will_be_alive = true;
            }
        } else if alive_cells_count == 3 {
            will_be_alive = true;
        }
        will_be_alive
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
    fn spawn_first_generation(field: &mut Vec<Vec<bool>>, field_size: usize) {
        let count_of_lifes = rand::random::<u8>() % 4 + 1;
        for _ in 0..count_of_lifes {
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
                    field[y_abs][x_abs] = true;
                }
            }
        }
    }
}
