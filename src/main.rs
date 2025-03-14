use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::{thread, time};

const WIDTH: usize = 20;
const HEIGHT: usize = 10;
const FROG_CHAR: char = '@';
const CAR_CHAR: char = '>';
const ROAD_CHAR: char = '-';
const WATER_CHAR: char = '~';

struct Game {
    frog_x: usize,
    frog_y: usize,
    cars: Vec<(usize, usize)>, // (x, y) positions of cars
    score: u32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        Game {
            frog_x: WIDTH / 2,
            frog_y: HEIGHT - 1,
            cars: vec![(0, 2), (5, 4), (10, 6)], // Initial car positions
            score: 0,
            game_over: false,
        }
    }

    fn update(&mut self) {
        // Move cars to the right
        for car in &mut self.cars {
            car.0 = (car.0 + 1) % WIDTH;
        }

        // Check for collisions
        for &(car_x, car_y) in &self.cars {
            if car_x == self.frog_x && car_y == self.frog_y {
                self.game_over = true;
            }
        }

        // Win condition: frog reaches the top
        if self.frog_y == 0 {
            self.score += 1;
            self.frog_x = WIDTH / 2;
            self.frog_y = HEIGHT - 1;
        }
    }

    fn move_frog(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up if self.frog_y > 0 => self.frog_y -= 1,
            KeyCode::Down if self.frog_y < HEIGHT - 1 => self.frog_y += 1,
            KeyCode::Left if self.frog_x > 0 => self.frog_x -= 1,
            KeyCode::Right if self.frog_x < WIDTH - 1 => self.frog_x += 1,
            _ => {}
        }
    }

    fn render(&self) {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        // Draw the game board
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut char_to_print = if y < HEIGHT / 2 { WATER_CHAR } else { ROAD_CHAR };
                if x == self.frog_x && y == self.frog_y {
                    char_to_print = FROG_CHAR;
                }
                for &(car_x, car_y) in &self.cars {
                    if x == car_x && y == car_y {
                        char_to_print = CAR_CHAR;
                    }
                }
                execute!(
                    stdout,
                    cursor::MoveTo(x as u16, y as u16),
                    Print(char_to_print)
                ).unwrap();
            }
        }

        // Display score and game over message
        execute!(
            stdout,
            cursor::MoveTo(0, HEIGHT as u16),
            Print(format!("Score: {}", self.score))
        ).unwrap();

        if self.game_over {
            execute!(
                stdout,
                cursor::MoveTo(0, HEIGHT as u16 + 1),
                SetForegroundColor(Color::Red),
                Print("GAME OVER! Press 'q' to quit."),
                ResetColor
            ).unwrap();
        }

        stdout.flush().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut game = Game::new();
    let tick_rate = Duration::from_millis(200); // Game speed

    'game_loop: loop {
        let now = Instant::now();

        // Handle input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break 'game_loop,
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                        if !game.game_over {
                            game.move_frog(key_event.code);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Update game state
        if !game.game_over {
            game.update();
        }

        // Render the game
        game.render();

        // Control frame rate
        let elapsed = now.elapsed();
        if elapsed < tick_rate {
            thread::sleep(tick_rate - elapsed);
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}