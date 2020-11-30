use rust_2048::board_engine::*;

use crossterm::event::{Event, KeyCode};
use crossterm::{event, terminal};

use std::time::Duration;
use std::error::Error;

enum UserInput {
    Up,
    Down,
    Left,
    Right,
    Quit,
    None
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut engine = BoardEngine::new();
    let mut player_won = false;
    
    'gameloop: loop {
        engine.print();

        if engine.is_game_over() {
            print!("Game Over!\r\n");
            break 'gameloop;
        }

        if !player_won {
            if engine.player_won() {
                player_won = true;
                print!("You win! You can continue playing to reach a higher score\r\n");
            }
        }

        let user_input = get_user_input().expect("Failed reading user input");
        match user_input {
            UserInput::Up => engine.move_up(),
            UserInput::Down => engine.move_down(),
            UserInput::Left => engine.move_left(),
            UserInput::Right => engine.move_right(),
            UserInput::Quit => break 'gameloop,
            _ => {}
        }
    }
    terminal::disable_raw_mode().unwrap();
}

fn get_user_input() -> Result<UserInput, Box<dyn Error>> {
    while event::poll(Duration::from_secs(10))? {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up | KeyCode::Char('w') => return Ok(UserInput::Up),
                KeyCode::Down | KeyCode::Char('s') => return Ok(UserInput::Down),
                KeyCode::Left | KeyCode::Char('a') => return Ok(UserInput::Left),
                KeyCode::Right | KeyCode::Char('d') => return Ok(UserInput::Right),
                KeyCode::Char('q') => return Ok(UserInput::Quit),
                _ => {}
            }
        }
    }

    return Ok(UserInput::None);
}
