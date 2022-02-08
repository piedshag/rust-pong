use crossterm::{ExecutableCommand};
use crossterm::terminal::{Clear, ClearType, size, SetSize, enable_raw_mode, disable_raw_mode};
use crossterm::style::{SetForegroundColor, Print, ResetColor, Color};
use crossterm::cursor::{Show, MoveTo, Hide};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent};

use std::time::{Duration, Instant};
use std::io::{Stdout, stdout};
use std::thread;

use rand::Rng;

pub struct Paddle {
    x: u16,
    y: u16,
    size: u16,
}

pub struct Ball {
    x: u16,
    y: u16,
    motion_x: i16,
    motion_y: i16,
}

impl Ball {
    pub fn step(&mut self) {
        self.x = (self.x as i16 + self.motion_x) as u16;
        self.y  = (self.y as i16 + self.motion_y) as u16;
    }
}

pub struct Game {
    stdout: Stdout,
    width: u16,
    height: u16,
    ball: Ball,
    paddle: Paddle,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        Self {
            stdout,
            width,
            height,
            ball: Ball {
                x: rand::thread_rng().gen_range(0..width),
                y: rand::thread_rng().gen_range(0..height),
                motion_x: 1,
                motion_y: 1,
            },
            paddle: Paddle {x : width/2, y: height - 2, size: 3},
        }
    }

    fn run(&mut self) {
        self.clear_ui();
        let mut done = false;
        while !done {
            let now = Instant::now();
            let interval = Duration::from_millis(30);
            while now.elapsed() < interval {
                let wait_for = interval - now.elapsed();
                if poll(wait_for).unwrap() {
                    let event = read().unwrap();
                    if let Event::Key(key_event) = event {
                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => done = true,
                            KeyCode::Right => self.paddle.x = self.paddle.x + 1,
                            KeyCode::Left => self.paddle.x = self.paddle.x - 1,
                            _ => ()
                        };
                    }
                }
            }
            self.check_collision();
            self.ball.step();
            self.render();
        }
    }

    fn check_collision(&mut self) {
        if self.ball.x % (self.width) == 0 {
            self.ball.motion_x = self.ball.motion_x * -1
        }

        if self.ball.y % (self.height + 3) == 0 {
            self.ball.motion_y = self.ball.motion_y * -1
        }

        if self.ball.y == self.paddle.y && ((self.paddle.x - self.paddle.size)..(self.paddle.x + self.paddle.size)).contains(&self.ball.x) {
            self.ball.motion_y = self.ball.motion_y * -1
        }
    }

    fn render(&mut self) {
        self.draw_background();
        self.draw_border();
        self.draw_ball();
        self.draw_paddle();
    }

    fn clear_ui(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Hide).unwrap();
    }

    fn draw_ball (&mut self) {
        self.stdout
            .execute(MoveTo(self.ball.x, self.ball.y)).unwrap()
            .execute(Print("O")).unwrap();
    }

    fn draw_background(&mut self) {
        self.stdout.execute(ResetColor).unwrap();

        for y in 0..self.height + 3 {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y)).unwrap()
                    .execute(Print(" ")).unwrap();
            }
        }
    }

    fn draw_border(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();

        for y in 0..self.height + 3 {
            self.stdout
                .execute(MoveTo(0, y)).unwrap()
                .execute(Print("█")).unwrap()
                .execute(MoveTo(self.width + 1, y)).unwrap()
                .execute(Print("█")).unwrap();
        }
    }

    fn draw_paddle(&mut self) {
        for x in (self.paddle.x - self.paddle.size)..(self.paddle.x + self.paddle.size) {
            self.stdout
                .execute(MoveTo(x, self.paddle.y)).unwrap()
                .execute(Print("▄")).unwrap();
        }
    }
}

fn main() {
    Game::new(stdout(), 25, 25).run()
}
