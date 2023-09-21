use macroquad::prelude::*;
pub fn draw_window() {
    clear_background(BLACK);
    draw_rectangles();
}
fn draw_rectangles() {
    let rectangle_y_pos = screen_height() / 4.0;
    let paddiing: f32 = 200.0;
    let left_offset: f32 = 100.0;
    let rectangle_height: f32 = 500.0;

    draw_rectangle_lines(
        left_offset,
        rectangle_y_pos,
        100.0,
        rectangle_height,
        5.0,
        WHITE,
    );
    draw_rectangle_lines(
        left_offset + paddiing,
        rectangle_y_pos,
        100.0,
        rectangle_height,
        5.0,
        WHITE,
    );
    draw_rectangle_lines(
        left_offset + (paddiing * 2.0),
        rectangle_y_pos,
        100.0,
        rectangle_height,
        5.0,
        WHITE,
    );
    draw_rectangle_lines(
        left_offset + (paddiing * 3.0),
        rectangle_y_pos,
        100.0,
        rectangle_height,
        5.0,
        WHITE,
    );
    draw_rectangle_lines(
        left_offset + (paddiing * 4.0),
        rectangle_y_pos,
        100.0,
        rectangle_height,
        5.0,
        WHITE,
    );
    draw_rectangle_lines(
        left_offset + (paddiing * 5.0),
        rectangle_y_pos,
        100.0,
        rectangle_height,
        5.0,
        WHITE,
    );
}
