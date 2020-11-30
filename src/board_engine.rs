extern crate rand;
use rand::Rng;
use super::{Board, BOARD_SIZE};

pub struct BoardEngine {
    board: Board
}

impl BoardEngine {
    pub fn new() -> Self {
        let board: Board = [[0; BOARD_SIZE]; BOARD_SIZE];

        let mut engine = Self {
            board
        };

        engine.gen_next_digit();
        engine
    }

    pub fn print(&self) {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");
        println!("");

        for row in self.board.iter() {
            for col in row.iter() {
                print!("{}\t", col);
            }
            print!("\r\n\r\n");
        }
        print!("Instructions: use arrows or wsad to move the board and q to quit\r\n");
    }

    pub fn move_up(&mut self) {
        for col in 0..BOARD_SIZE {
            
            for j in 1..BOARD_SIZE {
                for i in (1..j + 1).rev() {
                    if self.board[i][col] != 0 {
                        if self.board[i - 1][col] == 0 {
                            self.board[i - 1][col] = self.board[i][col];
                            self.board[i][col] = 0;
                        }

                        else if self.board[i - 1][col] == self.board[i][col] {
                            self.board[i - 1][col] *= 2;
                            self.board[i][col] = 0;
                            break;
                        }

                        else {
                            break;
                        }
                    }

                }
            }
        }
        self.gen_next_digit();
    }

    pub fn move_down(&mut self) {
        for col in 0..BOARD_SIZE {
            for j in (0..BOARD_SIZE - 1).rev() {
                for i in j..BOARD_SIZE - 1 {
                    if self.board[i][col] != 0 {
                        if self.board[i + 1][col] == 0 {
                            self.board[i + 1][col] = self.board[i][col];
                            self.board[i][col] = 0;
                        }

                        else if self.board[i + 1][col] == self.board[i][col] {
                            self.board[i + 1][col] *= 2;
                            self.board[i][col] = 0;
                            break;
                        }

                        else {
                            break;
                        }
                    }

                }
            }
        }
        self.gen_next_digit();
    }

    pub fn move_left(&mut self) {
        for row in self.board.iter_mut() {

            for i in 1..BOARD_SIZE {
                for j in (1..i + 1).rev() {
                    if row[j] != 0 {
                        if row[j - 1] == 0 {
                            row[j - 1] = row[j];
                            row[j] = 0;
                        }

                        else if row[j - 1] == row[j] {
                            row[j - 1] *= 2;
                            row[j] = 0;
                            break;
                        }

                        else {
                            break;
                        }
                    }
                }
            }
        }
        self.gen_next_digit();
    }

    pub fn move_right(&mut self) {
        for row in self.board.iter_mut() {

            for i in (0..BOARD_SIZE - 1).rev() {
                for j in i..BOARD_SIZE - 1 {
                    if row[j] != 0 {
                        if row[j + 1] == 0 {
                            row[j + 1] = row[j];
                            row[j] = 0;
                        }

                        else if row[j + 1] == row[j] {
                            row[j + 1] *= 2;
                            row[j] = 0;
                            break;
                        }

                        else {
                            break;
                        }
                    }
                }

            }
        }
        self.gen_next_digit();
    }

    pub fn is_game_over(&self) -> bool {
        let position = self.generate_random_position();

        match position {
            Some(_) => return false,
            None => return !self.has_available_moves(),
        }
    }

    pub fn player_won(&self) -> bool {
        for row in self.board.iter() {
            for col in row.iter() {
                if *col >= 2048 {
                    return true;
                }
            }
        }

        false
    }

    fn gen_next_digit(&mut self) {
        let position = self.generate_random_position();

        match position {
            Some(p) => self.board[p.0][p.1] = 2,
            None => {}
        }
    }

    fn get_available_indexes(&self) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();

        for (i, row) in self.board.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 0 {
                    result.push((i, j));
                }
            }
        }

        result
    }

    fn generate_random_position(&self) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let available_indexes = self.get_available_indexes();

        if available_indexes.len() == 0 {
            return None;
        }

        let position_index = rng.gen_range(0, available_indexes.len());         

        Some(available_indexes[position_index])
    }

    fn has_available_moves(&self) -> bool {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE - 1 {
                if self.board[i][j] == self.board[i][j + 1] {
                    return true;
                }

                else if self.board[j][i] == self.board[j + 1][i] {
                    return true;
                }
            }
        }

        false
    }

}
