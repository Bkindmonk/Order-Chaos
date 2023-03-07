use std::{thread, time};
use Player::{Chaos, Order};
use Tile::{Empty, Red};
use crate::players::Player;
use crate::{random_coordinates, random_pawn};
use crate::tile::Tile;

#[derive(Copy, Clone)]
pub struct GameState {
    pub board: [[Tile; 6]; 6],
    pub turn_player: Player,
}

impl GameState {
    pub const fn default_new() -> Self {
        Self {
            board: [[Empty; 6]; 6],
            turn_player: Order,
        }
    }

    pub fn play(&mut self, coordinates: (usize, usize), play: Tile) -> Result<&str, &str> {
        match self.board[coordinates.0][coordinates.1] {
            Empty => self.board[coordinates.0][coordinates.1] = play,
            _ => return Err("Space Already Occupied")
        }
        match self.turn_player {
            Chaos => self.turn_player = Order,
            Order => self.turn_player = Chaos
        }
        Ok("No Notes")
    }

    pub fn is_in_order(&self) -> bool {
        for row in 0..6 {
            if self.is_row_in_order(row) {
                return true;
            }
        }
        for column in 0..6 {
            if self.is_column_in_order(column) {
                return true;
            }
        }
        return self.is_down_diagonal_in_order(0, 0) ||
            self.is_down_diagonal_in_order(0, 1) ||
            self.is_down_diagonal_in_order(1, 0) ||
            self.is_up_diagonal_in_order(4, 0) ||
            self.is_up_diagonal_in_order(5, 0) ||
            self.is_up_diagonal_in_order(5, 1);
    }

    pub fn can_order_win(&self) -> bool {
        //TODO
        for row in self.board {
            for tile in row {
                if Empty.eq(&tile) {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_row_in_order(&self, row: usize) -> bool {
        let mut current_tile = Red;
        let mut streak = 0;
        let mut order_wins = false;
        for tile in self.board[row] {
            visit_tile_for_order(&mut current_tile, &tile, &mut streak, &mut order_wins);
        }
        return order_wins;
    }

    fn is_column_in_order(&self, column: usize) -> bool {
        let mut current_tile = Red;
        let mut streak = 0;
        let mut order_wins = false;
        for row in self.board {
            let tile = row[column];
            visit_tile_for_order(&mut current_tile, &tile, &mut streak, &mut order_wins);
        }
        return order_wins;
    }

    fn is_down_diagonal_in_order(&self, row: usize, column: usize) -> bool {
        let mut current_tile = Red;
        let mut streak = 0;
        let mut order_wins = false;
        let mut offset = 0;
        while row + offset < self.board.len() && column + offset < self.board[0].len() {
            let tile = self.board[row + offset][column + offset];
            visit_tile_for_order(&mut current_tile, &tile, &mut streak, &mut order_wins);
            offset += 1;
        }
        return order_wins;
    }

    fn is_up_diagonal_in_order(&self, row: usize, column: usize) -> bool {
        let mut current_tile = Red;
        let mut streak = 0;
        let mut order_wins = false;
        let mut offset = 0;
        while row >= offset && column + offset < self.board[0].len() {
            let tile = self.board[row - offset][column + offset];
            visit_tile_for_order(&mut current_tile, &tile, &mut streak, &mut order_wins);
            offset += 1;
        }
        return order_wins;
    }
}

fn visit_tile_for_order(current_tile: &mut Tile, visit: &Tile, streak: &mut i32, order_wins: &mut bool) {
    if current_tile.eq(&visit) {
        *streak += 1;
    } else if Empty.eq(&visit) {
        *streak = 0;
    } else {
        *current_tile = visit.clone();
        *streak = 1;
    }
    if *streak == 5 {
        *order_wins = true;
    }
    if *streak == 6 {
        *order_wins = false;
    }
}

pub fn make_a_random_move(game_state: &mut GameState) {
    let mut played: bool = false;
    thread::sleep(time::Duration::from_secs(3));
    while !played {
        let pawn = random_pawn();
        let coordinates = random_coordinates();
        let result = game_state.play(coordinates, pawn);
        played = result.is_ok();
    }
}

#[cfg(test)]
mod state_test {
    use crate::state::GameState;
    use crate::tile::Tile::{Blue, Red};

    #[test]
    fn test_5_horizontal() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((0, 1), Blue).expect("");
        game_state.play((0, 2), Blue).expect("");
        game_state.play((0, 3), Blue).expect("");
        game_state.play((0, 4), Blue).expect("");
        game_state.play((0, 5), Blue).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), true);
    }

    #[test]
    fn test_6_horizontal() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((0, 0), Blue).expect("");
        game_state.play((0, 1), Blue).expect("");
        game_state.play((0, 2), Blue).expect("");
        game_state.play((0, 3), Blue).expect("");
        game_state.play((0, 4), Blue).expect("");
        game_state.play((0, 5), Blue).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), false);
    }

    #[test]
    fn test_5_vertical() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((1, 1), Blue).expect("");
        game_state.play((2, 1), Blue).expect("");
        game_state.play((3, 1), Blue).expect("");
        game_state.play((4, 1), Blue).expect("");
        game_state.play((5, 1), Blue).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), true);
    }

    #[test]
    fn test_6_vertical() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((0, 1), Red).expect("");
        game_state.play((1, 1), Red).expect("");
        game_state.play((2, 1), Red).expect("");
        game_state.play((3, 1), Red).expect("");
        game_state.play((4, 1), Red).expect("");
        game_state.play((5, 1), Red).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), false);
    }

    #[test]
    fn test_5_down_diagonal() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((1, 1), Blue).expect("");
        game_state.play((2, 2), Blue).expect("");
        game_state.play((3, 3), Blue).expect("");
        game_state.play((4, 4), Blue).expect("");
        game_state.play((5, 5), Blue).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), true);
    }

    #[test]
    fn test_6_down_diagonal() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((0, 0), Blue).expect("");
        game_state.play((1, 1), Blue).expect("");
        game_state.play((2, 2), Blue).expect("");
        game_state.play((3, 3), Blue).expect("");
        game_state.play((4, 4), Blue).expect("");
        game_state.play((5, 5), Blue).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), false);
    }

    #[test]
    fn test_5_up_diagonal() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((1, 4), Red).expect("");
        game_state.play((2, 3), Red).expect("");
        game_state.play((3, 2), Red).expect("");
        game_state.play((4, 1), Red).expect("");
        game_state.play((5, 0), Red).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), true);
    }

    #[test]
    fn test_6_up_diagonal() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((0, 5), Red).expect("");
        game_state.play((1, 4), Red).expect("");
        game_state.play((2, 3), Red).expect("");
        game_state.play((3, 2), Red).expect("");
        game_state.play((4, 1), Red).expect("");
        game_state.play((5, 0), Red).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), false);
    }

    #[test]
    fn test_5_horizontal_6_vertical() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((0, 1), Blue).expect("");
        game_state.play((0, 2), Blue).expect("");
        game_state.play((0, 3), Blue).expect("");
        game_state.play((0, 4), Blue).expect("");
        game_state.play((0, 5), Blue).expect("");
        game_state.play((1, 1), Blue).expect("");
        game_state.play((2, 1), Blue).expect("");
        game_state.play((3, 1), Blue).expect("");
        game_state.play((4, 1), Blue).expect("");
        game_state.play((5, 1), Blue).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), true);
    }

    #[test]
    fn test_6_horizontal_5_vertical() {
        let mut game_state: GameState = GameState::default_new();
        game_state.play((0, 0), Red).expect("");
        game_state.play((0, 1), Red).expect("");
        game_state.play((0, 2), Red).expect("");
        game_state.play((0, 3), Red).expect("");
        game_state.play((0, 4), Red).expect("");
        game_state.play((0, 5), Red).expect("");
        game_state.play((1, 1), Red).expect("");
        game_state.play((2, 1), Red).expect("");
        game_state.play((3, 1), Red).expect("");
        game_state.play((4, 1), Red).expect("");
        println!("{}", game_state);
        println!("Order Won: {}", game_state.is_in_order());
        assert_eq!(game_state.is_in_order(), true);
    }
}