use piston_window::{PistonWindow, WindowSettings};

pub struct Display {
    width: u32,
    height: u32,
    pub window: PistonWindow,
}

impl Display {
    pub fn new(chip8_width: u32, chip8_height: u32, chip8_scale: u32, title: &str) -> Display {
        let mut window = WindowSettings::new(
            title,
            [chip8_width * chip8_scale, chip8_height * chip8_scale],
        )
        .exit_on_esc(true)
        .build()
        .unwrap();
        Display {
            width: chip8_width * chip8_scale,
            height: chip8_height * chip8_scale,
            window: window,
        }
    }
}
