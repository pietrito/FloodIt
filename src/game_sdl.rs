use crate::game::Game;
use crate::utils;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub enum GameSDLError {
    ResolutionWidthError,
    ResolutionHeightError,
    DrawGameError,
}

impl std::error::Error for GameSDLError {}

impl fmt::Display for GameSDLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameSDLError::ResolutionWidthError => {
                write!(f, "Resolution X must be a multiple of width.")
            }
            GameSDLError::ResolutionHeightError => {
                write!(f, "Resolution Y must be a multiple of height.")
            }
            GameSDLError::DrawGameError => {
                write!(f, "Error whilst drawing the game.")
            }
        }
    }
}

pub struct GameSDL {
    size_x: u32,
    size_y: u32,

    colors: Vec<Color>,

    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,

    game: Game,
}

impl GameSDL {
    pub fn new(
        title: Option<String>,
        res_x: u32,
        res_y: u32,
        width: usize,
        height: usize,
        difficulty: u8,
    ) -> Result<Self, GameSDLError> {
        if res_x % (width as u32) != 0 {
            return Err(GameSDLError::ResolutionWidthError);
        }

        let size_x = res_x / width as u32;

        if res_y % (height as u32) != 0 {
            return Err(GameSDLError::ResolutionHeightError);
        }

        let size_y = res_y / height as u32;

        let mut colors = vec![Color::RGB(124, 0, 0); difficulty as usize];
        for i in 0..difficulty as usize {
            colors[i] = utils::ColorHSV(360.0f32 / (difficulty as f32) * (i as f32), 1f32, 1f32)
                .to_sdl_color();
        }

        let game = Game::new(width, height, difficulty);

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(&title.unwrap_or("Rust Flood-It".to_string()), res_x, res_y)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Ok(GameSDL {
            size_x,
            size_y,
            colors,
            canvas,
            event_pump,
            game,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
        'running: loop {
            self.canvas.clear();
            self.draw_game()?;
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn draw_game(&mut self) -> Result<(), String> {
        for y in 0..self.game.board.height {
            for x in 0..self.game.board.width {
                self.canvas
                    .set_draw_color(self.colors[self.game.board.cases[y][x].color as usize]);
                self.canvas.fill_rect(Rect::new(
                    (x as u32 * self.size_x) as i32,
                    (y as u32 * self.size_y) as i32,
                    self.size_x,
                    self.size_y,
                ))?;
            }
        }

        Ok(())
    }
}
