use piston_window::{
    clear, rectangle, types::Color, Context, G2d, Graphics, PistonWindow, WindowSettings,
};

use crate::{BACKCOLOR, FRONTCOLOR};

use super::Cpu;

pub struct Display {
    width: u32,
    height: u32,
    pub scale: u32,
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
            scale: chip8_scale,
            window: window,
        }
    }

    pub fn draw(&mut self, cpu: &Cpu, e: &piston_window::Event) {
        self.window.draw_2d(e, |c, g, _| {
            clear(BACKCOLOR, g);
            for y in 0..cpu.video_ram.len() {
                for x in 0..cpu.video_ram[y].len() {
                    if cpu.video_ram[y][x] == 1 {
                        rectangle(
                            FRONTCOLOR,
                            [
                                x as f64 * self.scale as f64,
                                y as f64 * self.scale as f64,
                                self.scale as f64,
                                self.scale as f64,
                            ],
                            c.transform,
                            g,
                        );
                    }
                }
            }
        });
    }
}
