use piston_window::{types::Color, *};

mod emulation;

struct Config {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub ram_size: usize,
}

const DEFAULT_CONFIG: Config = Config {
    width: 64,
    height: 32,
    scale: 16,
    ram_size: 4096,
};

// 26 28 44
const BACKCOLOR: Color = [0.1, 0.11, 0.17, 1.0];
// 37 113 121
const FRONTCOLOR: Color = [0.14, 0.44, 0.47, 1.0];

fn main() {
    let mut display = emulation::Display::new(
        DEFAULT_CONFIG.width,
        DEFAULT_CONFIG.height,
        DEFAULT_CONFIG.scale,
        "Chip8 Emulator",
    );

    let disk = emulation::Disk::new("roms/IBM_Logo.ch8");
    disk.print_disk();

    let mut cpu = emulation::Cpu::new();
    cpu.load_disk_to_ram(&disk);

    while let Some(e) = display.window.next() {
        // Handle cpu
        cpu.next();

        // Handle graphics

        display.window.draw_2d(&e, |c, g, _d| {
            clear(BACKCOLOR, g);
            rectangle(FRONTCOLOR, [0.0, 0.0, 24.0, 24.0], c.transform, g);
        });
    }
}