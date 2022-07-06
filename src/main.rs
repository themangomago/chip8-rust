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

    let disk = emulation::Disk::new("roms/Maze_[David Winter, 199x].ch8");
    disk.print_disk();

    let mut cpu = emulation::Cpu::new();
    cpu.load_disk_to_ram(&disk);

    while let Some(e) = display.window.next() {
        // Handle input
        //TODO: Handle input

        // Handle cpu
        cpu.next();

        // Handle display
        display.draw(&cpu, &e);
    }
}
