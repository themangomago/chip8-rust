use super::Disk;

#[cfg(test)]
#[path = "./tests/cpu.rs"]
mod tests;

const CPU_DEBUG_PRINT: bool = false;
const CPU_DEBUG_PRINT_VIDEO_RAM: bool = true;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    // Memory access
    pub video_ram: [[u8; 64]; 32],
    pub video_ram_changed: bool,
    ram: [u8; 4096],
    // Registers
    pub reg_v: [u8; 16],
    pub reg_i: u16,
    pub reg_pc: u16,
    pub reg_sp: u8,
    // Stack
    pub stack: [u16; 16],
    pub stack_ptr: u8,
    // Opcodes
    pub opcode: u16,
    pub opcode_last: u16,
    // Timers
    pub reg_delay_timer: u8,
    pub reg_sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            // Memory access
            video_ram: [[0; 64]; 32],
            video_ram_changed: true,
            ram: [0; 4096],
            // Registers
            reg_v: [0; 16],
            reg_i: 0,
            reg_pc: 0,
            reg_sp: 0,
            // Stack
            stack: [0; 16],
            stack_ptr: 0,
            // Opcodes
            opcode: 0,
            opcode_last: 0,
            // Timers
            reg_delay_timer: 0,
            reg_sound_timer: 0,
        };

        cpu.reg_pc = 0x200;

        for i in 0..FONT_SET.len() {
            cpu.ram[i] = FONT_SET[i];
        }
        return cpu;
    }

    pub fn load_disk_to_ram(&mut self, disk: &Disk) {
        for i in 0..disk.size {
            self.ram[i as usize + 0x200] = disk.rom[i as usize];
        }
        //TODO: someone we need to do a disk size check because of the +0x200
        println!("Loaded {} bytes to RAM", disk.size);
    }

    pub fn next(&mut self) {
        self.opcode_last = self.opcode;

        if self.reg_pc == 0x1000 {
            //TODO: Overflow check
            return;
        }

        self.opcode = (self.ram[self.reg_pc as usize] as u16) << 8
            | self.ram[(self.reg_pc + 1) as usize] as u16;
        self.reg_pc += 2;
        self.execute();
    }

    fn execute(&mut self) {
        // if self.opcode != 0 {
        //     println!("Opcode: {:04x}", self.opcode);
        // }

        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x0FFF {
                0x00E0 => self.op_0x00e0(),
                0x00EE => self.op_0x00ee(),
                _ => (), // Noop //TODO handle 0NNN calls
            },
            0x1000 => self.op_0x1nnn(),
            0x2000 => self.op_0x2nnn(),
            0x3000 => self.op_0x3xkk(),
            0x4000 => self.op_0x4xkk(),
            0x5000 => self.op_0x5xy0(),
            0x6000 => self.op_0x6xkk(),
            0x7000 => self.op_0x7xkk(),
            0x8000 => match self.opcode & 0x000F {
                0x0000 => self.op_0x8xy0(),
                0x0001 => self.op_0x8xy1(),
                0x0002 => self.op_0x8xy2(),
                0x0003 => self.op_0x8xy3(),
                //     0x0004 => self.op_0x8xy4(),
                //     0x0005 => self.op_0x8xy5(),
                //     0x0006 => self.op_0x8xy6(),
                //     0x0007 => self.op_0x8xy7(),
                //     0x000E => self.op_0x8xyE(),
                _ => println!("Unknown opcode: {:04x}", self.opcode),
            },
            // 0x9000 => self.op_0x9xy0(),
            0xA000 => self.op_0xAnnn(),
            // 0xB000 => self.op_0xBnnn(),
            // 0xC000 => self.op_0xCxkk(),
            0xD000 => self.op_0xDxyn(),
            // 0xE000 => match self.opcode & 0x000F {
            //     0x000E => self.op_0xEx9E(),
            //     0x000A => self.op_0xExA1(),
            //     _ => println!("Unknown opcode: {:04x}", self.opcode),
            // },
            _ => println!("Unknown opcode: {:04x}", self.opcode),
        }
    }

    // Clear display
    fn op_0x00e0(&mut self) {
        for i in 0..64 {
            for j in 0..32 {
                self.video_ram[j][i] = 0;
            }
        }
        self.video_ram_changed = true;
        debug_print(self.opcode);
    }

    // Return from subroutine
    fn op_0x00ee(&mut self) {
        self.reg_sp -= 1;
        self.reg_pc = self.stack[self.reg_sp as usize];
        debug_print(self.opcode);
    }

    // Jump to address NNN
    fn op_0x1nnn(&mut self) {
        self.reg_pc = self.opcode & 0x0FFF;
        //debug_print(self.opcode);
    }

    // Call subroutine at NNN
    fn op_0x2nnn(&mut self) {
        self.stack[self.reg_sp as usize] = self.reg_pc;
        self.reg_sp += 1;
        self.reg_pc = self.opcode & 0x0FFF;
        debug_print(self.opcode);
    }

    // Skip next instruction if Vx = kk
    fn op_0x3xkk(&mut self) {
        if self.reg_v[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x00FF) as u8 {
            self.reg_pc += 2;
        }
        debug_print(self.opcode);
    }

    // Skip next instruction if Vx != kk
    fn op_0x4xkk(&mut self) {
        if self.reg_v[((self.opcode & 0x0F00) >> 8) as usize] != (self.opcode & 0x00FF) as u8 {
            self.reg_pc += 2;
        }
        debug_print(self.opcode);
    }

    // Skip next instruction if Vx = Vy
    fn op_0x5xy0(&mut self) {
        if self.reg_v[((self.opcode & 0x0F00) >> 8) as usize]
            == self.reg_v[((self.opcode & 0x00F0) >> 4) as usize]
        {
            self.reg_pc += 2;
        }
        debug_print(self.opcode);
    }

    // Set Vx = kk
    fn op_0x6xkk(&mut self) {
        self.reg_v[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
        debug_print(self.opcode);
    }

    // Set Vx = Vx + kk
    fn op_0x7xkk(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_kk = (self.opcode & 0x00FF) as u8;
        self.reg_v[reg_x] = self.reg_v[reg_x].wrapping_add(reg_kk);
        debug_print(self.opcode);
    }

    // Set Vx = Vy
    fn op_0x8xy0(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.reg_v[reg_x] = self.reg_v[reg_y];
        debug_print(self.opcode);
    }

    // Set Vx = Vx OR Vy
    fn op_0x8xy1(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.reg_v[reg_x] = self.reg_v[reg_x] | self.reg_v[reg_y];
        debug_print(self.opcode);
    }

    // Set Vx = Vx AND Vy
    fn op_0x8xy2(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.reg_v[reg_x] = self.reg_v[reg_x] & self.reg_v[reg_y];
        debug_print(self.opcode);
    }

    // Set Vx = Vx XOR Vy
    fn op_0x8xy3(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
        self.reg_v[reg_x] = self.reg_v[reg_x] ^ self.reg_v[reg_y];
        debug_print(self.opcode);
    }

    // Sets I to the address NNN.
    fn op_0xAnnn(&mut self) {
        self.reg_i = self.opcode & 0x0FFF;
        debug_print(self.opcode);
    }

    // Draws a sprite at coordinate (Vx, Vy) with width 8 pixels and height N pixels.
    fn op_0xDxyn(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let height = (self.opcode & 0x000F) as usize;

        self.reg_v[0xF] = 0;

        for i in 0..height {
            let sprite_line = self.ram[self.reg_i as usize + i];
            for j in 0..8 {
                let pixel = (sprite_line >> (7 - j)) & 0x1;
                let x = (self.reg_v[vx] as usize + j) % 64;
                let y = (self.reg_v[vy] as usize + i) % 32;
                if pixel == 1 && self.video_ram[y][x] == 1 {
                    self.reg_v[0xF] = 1;
                }
                self.video_ram[y][x] ^= pixel;
            }
        }
        //TODO: implement this
        debug_print(self.opcode);
        debug_print_video_ram(&self.video_ram);
    }
}

fn debug_print(opcode: u16) {
    if CPU_DEBUG_PRINT {
        println!("Opcode: {:04x}", opcode);
    }
}

fn debug_print_video_ram(video_ram: &[[u8; 64]; 32]) {
    if CPU_DEBUG_PRINT_VIDEO_RAM {
        for i in 0..32 {
            for j in 0..64 {
                print!("{}", video_ram[i][j]);
            }
            println!();
        }
    }
}
