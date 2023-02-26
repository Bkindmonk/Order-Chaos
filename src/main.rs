use crate::players::Player::{Chaos, Order};
use crate::state::{GameState, make_a_random_move};
use crate::tile::Tile::{Blue, Red};
use rand::Rng;
use std::io;
use regex::Regex;
use crate::config::read_options;
use crate::display::{clear_output, show_error_message, show_exit_dialog, show_game_state, show_input_prompt, show_welcome_text, show_winner};
use crate::gui::show_main_screen;
use crate::tile::Tile;

mod tile;
mod state;
mod players;
mod config;
mod display;
mod gui;


fn main() {
    read_options();
    if config::get().disable_gui {
        welcome_screen();
        let mut game_state = GameState::default_new();
        show_game_state(&game_state);
        game_loop(&mut game_state, config::get().ai_vs_ai_demo);
        end_of_game_screen();
    } else {
        show_main_screen();
    }
}

fn welcome_screen() {
    show_welcome_text();
    read_console().unwrap();
    clear_output();
}

fn end_of_game_screen() {
    show_exit_dialog();
    read_console().unwrap();
}

fn game_loop(game_state: &mut GameState, ai_vs_ai: bool) {
    while game_state.can_order_win() && !game_state.is_in_order() {
        if ai_vs_ai {
            make_a_random_move(game_state);
        } else {
            ask_for_a_move(game_state);
        }
        clear_output();
        show_game_state(game_state);
    }
    if game_state.is_in_order() {
        show_winner(Order);
    } else {
        show_winner(Chaos);
    }
}

fn random_pawn() -> Tile {
    if rand::random() {
        return Blue;
    }
    return Red;
}

fn random_coordinates() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    return (rng.gen_range(0..6), rng.gen_range(0..6));
}

fn ask_for_a_move(game_state: &mut GameState) {
    let mut played: bool = false;
    while !played {
        show_input_prompt();
        let input;
        let result_in = read_input();
        match result_in {
            Err(v) => {
                show_error_message(v);
                continue;
            }
            Ok(v) => input = v
        }

        let result = game_state.play(input.1, input.0);
        match result {
            Err(v) => show_error_message(&format!("That was not a legal move. Message: {}", v)),
            _ => played = true
        }
    }
}

fn read_input() -> Result<(Tile, (usize, usize)), &'static str> {
    let regular_expression_1 = Regex::new(r"([X,O]) ([A-F])([1-6])").unwrap();
    let regular_expression_2 = Regex::new(r"([X,O]) ([1-6])([A-F])").unwrap();

    let user_input;
    let io_result = read_console();
    match io_result {
        Err(v) => return Err(v),
        Ok(v) => user_input = v
    }

    if regular_expression_1.is_match(&user_input) {
        return parse_input(&user_input, true);
    } else if regular_expression_2.is_match(&user_input) {
        return parse_input(&user_input, false);
    }

    show_error_message(&format!("None of the Regular Expressions matched the input: {}", user_input));
    return Err("None of the Regular Expressions matched the input");
}

fn read_console() -> Result<String, &'static str> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let io_result = stdin.read_line(&mut user_input);
    if io_result.is_err() {
        return Err("Error Reading from stdin");
    }
    return Ok(user_input);
}

fn parse_input(input: &str, letter_first: bool) -> Result<(Tile, (usize, usize)), &'static str> {
    let tile;
    let split: Vec<&str> = input.split("").collect();
    if split[1].eq_ignore_ascii_case("X") {
        tile = Red;
    } else {
        tile = Blue;
    }
    let column: usize;
    let row: usize;
    let letter: &str;
    let number: &str;
    if letter_first {
        letter = split[3];
        number = split[4];
    } else {
        letter = split[4];
        number = split[3];
    }

    row = number.parse().unwrap();
    match letter {
        "A" => {
            column = 0;
        }
        "B" => {
            column = 1;
        }
        "C" => {
            column = 2;
        }
        "D" => {
            column = 3;
        }
        "E" => {
            column = 4;
        }
        "F" => {
            column = 5;
        }
        _ => {
            return Err("Not a valid column");
        }
    }

    return Ok((tile, (row - 1, column)));
}