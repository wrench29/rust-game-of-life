mod field;
mod game;

fn main() {
    let cells_count = 200;
    let mut game_field = field::GameField::new(cells_count);
    let mut game_logic = game::GameLogic::new(cells_count);

    let mut generation = 1;
    let mut generation_per_second: usize = 1;
    while !game_field.should_close() {
        let cells = game_logic.get_cells();
        game_field.draw_cells(&cells, generation, generation_per_second);
        if game_logic.update_cells() {
            generation += 1;
        }
        if game_field.is_up_pressed() && generation_per_second < 20 {
            generation_per_second += 1;
            game_logic.set_update_speed(generation_per_second);
        }
        if game_field.is_down_pressed() && generation_per_second > 1 {
            generation_per_second -= 1;
            game_logic.set_update_speed(generation_per_second);
        }
    }
}
