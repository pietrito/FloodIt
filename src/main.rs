extern crate sdl2;

use floodit::game_sdl;

use config::{Config, File, FileFormat};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

const CONFIG_PATH: &str = "config/default.json";

pub fn main() -> Result<()> {
    let builder = Config::builder().add_source(File::new(CONFIG_PATH, FileFormat::Json));

    let config = match builder.build() {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "Error while loading the configuration file {}: {}",
                CONFIG_PATH, e
            );

            return Ok(());
        }
    };

    let title = match config.get_string("title") {
        Ok(title) => Some(title),
        Err(_) => None,
    };

    let res_x = match config.get_int("res_x") {
        Ok(res_x) => res_x as u32,
        Err(e) => {
            eprintln!(
                "Error while loading configuration file {}: {}",
                CONFIG_PATH, e
            );
            return Ok(());
        }
    };

    let res_y = match config.get_int("res_y") {
        Ok(res_y) => res_y as u32,
        Err(e) => {
            eprintln!(
                "Error while loading configuration file {}: {}",
                CONFIG_PATH, e
            );
            return Ok(());
        }
    };

    let width = match config.get_int("width") {
        Ok(width) => width as usize,
        Err(e) => {
            eprintln!(
                "Error while loading configuration file {}: {}",
                CONFIG_PATH, e
            );
            return Ok(());
        }
    };

    let height = match config.get_int("height") {
        Ok(height) => height as usize,
        Err(e) => {
            eprintln!(
                "Error while loading configuration file {}: {}",
                CONFIG_PATH, e
            );
            return Ok(());
        }
    };

    let difficulty = match config.get_int("difficulty") {
        Ok(difficulty) => difficulty as u8,
        Err(e) => {
            eprintln!(
                "Error while loading configuration file {}: {}",
                CONFIG_PATH, e
            );
            return Ok(());
        }
    };

    let mut game_sdl = game_sdl::GameSDL::new(title, res_x, res_y, width, height, difficulty)?;

    game_sdl.run()?;

    Ok(())
}
