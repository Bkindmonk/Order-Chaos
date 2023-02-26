use std::fmt;
use std::fmt::Formatter;
use crate::config;
use crate::players::Player;
use crate::players::Player::{Chaos, Order};
use crate::state::GameState;
use crate::tile::Tile::{Blue, Red, Empty};
use crate::tile::Tile;




pub fn show_welcome_text() {
    let line_1;
    let line_2: String;
    let line_3 = "Order plays first, then turns alternate.";
    let line_4: String;
    let line_5: String;
    let line_6: String;
    let line_7 = "Six-in-a-row does not qualify as a win\n\n";
    let line_8 = "Press any key to continue.";

    if config::get().disable_color_and_emoji {
        line_1 = "\n\n\nWelcome to the ORDER & CHAOS electronic simulator.\n\n\n";
        line_2 = "Both players control both sets of pieces (X and O). The game starts with the board empty.".to_string();
        line_4 = "On each turn, a player places either an X or an O on any open square. Once played, pieces cannot be moved".to_string();
        line_5 = "Order aims to get exactly five like pieces in a row either vertically, horizontally, or diagonally.".to_string();
        line_6 = "Chaos aims to fill the board without completion of a line of five like pieces.".to_string();
    } else if config::get().disable_emoji {
        line_1 = "\n\n\nWelcome to the \x1b[33mORDER\x1b[0m & \x1b[32mCHAOS\x1b[0m electronic simulator.\n\n\n";
        line_2 = "Both players control both sets of pieces (\x1b[31mX\x1b[0m and \x1b[34mO\x1b[0m). The game starts with the board empty.".to_string();
        line_4 = "On each turn, a player places either an \x1b[31mX\x1b[0m or an \x1b[34mO\x1b[0m on any open square. Once played, pieces cannot be moved".to_string();
        line_5 = format!("{} aims to get exactly five like pieces in a row either vertically, horizontally, or diagonally.", player_fmt(&Order));
        line_6 = format!("{} aims to fill the board without completion of a line of five like pieces.", player_fmt(&Chaos));
    } else {
        line_1 = "\n\n\nWelcome to the \x1b[33mＯＲＤＥＲ\x1b[0m　＆　\x1b[32mＣＨＡＯＳ\x1b[0m electronic simulator.\n\n\n";
        line_2 = format!("Both players control both sets of pieces ({} and {}). The game starts with the board empty.", tile_fmt(&Red), tile_fmt(&Blue));
        line_4 = format!("On each turn, a player places either an {} or an {} on any open square. Once played, pieces cannot be moved", tile_fmt(&Red), tile_fmt(&Blue));
        line_5 = format!("{} aims to get exactly five like pieces in a row either vertically, horizontally, or diagonally.", player_fmt(&Order));
        line_6 = format!("{} aims to fill the board without completion of a line of five like pieces.", player_fmt(&Chaos));
    }
    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", line_1, line_2, line_3, line_4, line_5, line_6, line_7, line_8);
}

pub fn show_exit_dialog() {
    println!("Press any key to exit the game.");
}

pub fn show_winner(winner: Player) {
    println!("{} Won!", player_fmt(&winner));
}

pub fn show_game_state(game_state: &GameState) {
    println!("{}", game_state);
}

pub fn show_input_prompt() {
    println!("Please select which pawn should be placed in which location on the board.\n");
    println!("Examples: \nX A1\nO 5F\n");
}

pub fn show_error_message(e: &str) {
    println!("{}", e);
}

pub fn clear_output() {
    let term = console::Term::stdout();
    let _ = term.clear_screen();
}

fn player_fmt(player: &Player) -> &'static str {
    if config::get().disable_color_and_emoji {
        return match *player {
            Order => "Order",
            Chaos => "Chaos"
        };
    }
    match *player {
        Order => "\x1b[33mOrder\x1b[0m",
        Chaos => "\x1b[32mChaos\x1b[0m"
    }
}

fn tile_fmt(tile: &Tile) -> &str {
    if config::get().disable_color_and_emoji || config::get().disable_emoji {
        return match tile {
            Empty => "[ ]",
            Blue => " O ",
            Red => " X "
        };
    }

    match tile {
        Empty => "🟪",
        Blue => "🔵",
        Red => "❌"
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut game_board = String::from("");
        self.show_game_board(&mut game_board);
        self.show_turn_player(&mut game_board);
        write!(f, "{}", game_board)
    }
}

impl GameState {
    fn show_column_labels(game_board: &mut String) {
        if !(config::get().disable_color_and_emoji || config::get().disable_emoji) {
            game_board.push_str(" ＡＢＣＤＥＦ\n");
        } else {
            game_board.push_str("  A  B  C  D  E  F\n");
        }
    }

    fn get_turn_color(&self) -> &str {
        if Order.eq(&self.turn_player) && !config::get().disable_color_and_emoji {
            "\x1b[43m"
        } else {
            "\x1b[42m"
        }
    }

    fn show_board_with_turn_colors(game_board: &mut String, turn_color: &str, row: &[Tile; 6]) {
        game_board.push_str(turn_color);
        for tile in row {
            game_board.push_str(tile_fmt(tile));
        }
        game_board.push_str("\x1b[0m");
    }

    fn show_plain_board(game_board: &mut String, row: &[Tile; 6]) {
        for tile in row {
            game_board.push_str(tile_fmt(tile));
        }
    }

    fn show_turn_player(&self, game_board: &mut String) {
        game_board.push_str("\nTurn Player: ");
        game_board.push_str(player_fmt(&self.turn_player));
    }

    fn show_game_board(&self, game_board: &mut String) {
        let turn_color = self.get_turn_color();
        Self::show_column_labels(game_board);
        for (index, row) in self.board.iter().enumerate() {
            game_board.push_str(&(index + 1).to_string());
            if config::get().disable_color_and_emoji {
                Self::show_plain_board(game_board, row);
            } else {
                Self::show_board_with_turn_colors(game_board, turn_color, row);
            }
            game_board.push('\n');
        }
    }
}