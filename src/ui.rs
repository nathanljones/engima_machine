use crate::common::ASCII_OFFSET;
use macroquad::prelude::*;
pub fn draw_window() {
    clear_background(BLACK);
    draw_rectangles();
    draw_headings();
    draw_plugboard_chars();
    draw_rotor1_chars();
    draw_rotor2_chars();
    draw_rotor3_chars();
    draw_reflector_chars();
}
fn draw_rectangles() {
    let rectangle_y_pos = screen_height() / 4.0;
    let paddiing: f32 = 200.0;
    let left_offset: f32 = 100.0;
    let rectangle_height: f32 = 550.0;
    for rects in 0..5 {
        draw_rectangle_lines(
            left_offset + (paddiing * rects as f32),
            rectangle_y_pos,
            100.0,
            rectangle_height,
            5.0,
            WHITE,
        );
    }
}
fn draw_headings() {
    let ypos: f32 = 200.0;
    let font_size: f32 = 20.0;

    draw_text("Plugboard", 110.0, ypos, font_size, WHITE);
    draw_text("Rotor 1", 310.0, ypos, font_size, WHITE);
    draw_text("Rotor 2", 510.0, ypos, font_size, WHITE);
    draw_text("Rotor 3", 710.0, ypos, font_size, WHITE);
    draw_text("Reflector", 910.0, ypos, font_size, WHITE);
}
fn draw_plugboard_chars() {
    let font_size: f32 = 27.0;
    let initial_y_pos = screen_height() / 3.5;
    let x_col1_char_pos = 110.0;
    let x_col2_char_pos = 170.0;

    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col1_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col2_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
}
fn draw_rotor1_chars() {
    let font_size: f32 = 27.0;
    let initial_y_pos = screen_height() / 3.5;
    let x_col1_char_pos = 310.0;
    let x_col2_char_pos = 370.0;

    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col1_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col2_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
}
fn draw_rotor2_chars() {
    let font_size: f32 = 27.0;
    let initial_y_pos = screen_height() / 3.5;
    let x_col1_char_pos = 510.0;
    let x_col2_char_pos = 570.0;

    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col1_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col2_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
}
fn draw_rotor3_chars() {
    let font_size: f32 = 27.0;
    let initial_y_pos = screen_height() / 3.5;
    let x_col1_char_pos = 710.0;
    let x_col2_char_pos = 770.0;

    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col1_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col2_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
}
fn draw_reflector_chars() {
    let font_size: f32 = 27.0;
    let initial_y_pos = screen_height() / 3.5;
    let x_col1_char_pos = 910.0;
    let x_col2_char_pos = 970.0;

    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col1_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
    for character in 0..26 {
        let letter = (char::from_u32(character + ASCII_OFFSET))
            .unwrap()
            .to_string();
        draw_text(
            &letter,
            x_col2_char_pos,
            initial_y_pos + (character * 20) as f32,
            font_size,
            WHITE,
        );
    }
}
