use super::Disk;
use rand::Rng;

#[cfg(test)]
#[path = "./tests/cpu.rs"]
mod tests;

const CPU_DEBUG_PRINT: bool = true;
const CPU_DEBUG_PRINT_VIDEO_RAM: bool = false;

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
    // Keyboard
    pub keyboard: [bool; 16],
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
            // Keyboard
            keyboard: [false; 16],
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

    pub fn key_pressed(&mut self, key: u8) {
        self.keyboard[key as usize - 1] = true;
        println!("Key pressed: {}", key);
    }

    pub fn next(&mut self) {
        self.opcode_last = self.opcode;

        if self.reg_pc == 0x1000 {
            //TODO: Overflow check
            return;
        }

        // Run opcode
        self.opcode = (self.ram[self.reg_pc as usize] as u16) << 8
            | self.ram[(self.reg_pc + 1) as usize] as u16;
        self.reg_pc += 2;
        self.execute();

        // Handle timers
        if self.reg_delay_timer > 0 {
            self.reg_delay_timer = self.reg_delay_timer - 1;
        }

        if self.reg_sound_timer > 0 {
            self.reg_sound_timer = self.reg_sound_timer - 1;
        }
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
                0x0004 => self.op_0x8xy4(),
                0x0005 => self.op_0x8xy5(),
                0x0006 => self.op_0x8xy6(),
                0x0007 => self.op_0x8xy7(),
                0x000E => self.op_0x8xyE(),
                _ => println!("0x8000 Unknown opcode: {:04x}", self.opcode),
            },
            0x9000 => self.op_0x9xy0(),
            0xA000 => self.op_0xAnnn(),
            0xB000 => self.op_0xBnnn(),
            0xC000 => self.op_0xCxkk(),
            0xD000 => self.op_0xDxyn(),
            0xE000 => match self.opcode & 0x000F {
                0x000E => self.op_0xEx9E(),
                0x0001 => self.op_0xExA1(),
                _ => println!("0xE000 Unknown opcode: {:04x}", self.opcode),
            },
            0xF000 => match self.opcode & 0x00FF {
                0x0007 => self.op_0xFx07(),
                0x000A => self.op_0xFx0A(),
                0x0015 => self.op_0xFx15(),
                0x0018 => self.op_0xFx18(),
                0x001E => self.op_0xFx1E(),
                0x0029 => self.op_0xFx29(),
                0x0033 => self.op_0xFx33(),
                0x0055 => self.op_0xFx55(),
                0x0065 => self.op_0xFx65(),
                _ => println!("0xF000 Unknown opcode: {:04x}", self.opcode),
            },
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
        println!("cleared");
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

    // Set Vx = Vx + Vy, set VF = carry
    fn op_0x8xy4(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;

        let (result, carry) = self.reg_v[reg_x].overflowing_add(self.reg_v[reg_y]);
        self.reg_v[reg_x] = result;
        self.reg_v[0xF] = if carry { 1 } else { 0 };

        debug_print(self.opcode);
    }

    // Set Vx = Vx - Vy, set VF = NOT borrow
    fn op_0x8xy5(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
        let (result, borrow) = self.reg_v[reg_x].overflowing_sub(self.reg_v[reg_y]);
        self.reg_v[reg_x] = result;
        self.reg_v[0xF] = if borrow { 0 } else { 1 };
        debug_print(self.opcode);
    }

    // Set Vx = Vx SHIFT RIGHT 1, set VF = least significant bit of Vx before shift
    fn op_0x8xy6(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_v[0xF] = self.reg_v[reg_x] & 0x1;
        self.reg_v[reg_x] = self.reg_v[reg_x] >> 1;
        debug_print(self.opcode);
    }

    // Set Vx = Vy - Vx, set VF = NOT borrow
    fn op_0x8xy7(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
        let (result, borrow) = self.reg_v[reg_y].overflowing_sub(self.reg_v[reg_x]);
        self.reg_v[reg_x] = result;
        self.reg_v[0xF] = if borrow { 0 } else { 1 };
        debug_print(self.opcode);
    }

    // Set Vx = Vx SHIFT LEFT 1, set VF = most significant bit of Vx before shift
    fn op_0x8xyE(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_v[0xF] = (self.reg_v[reg_x] & 0x80) >> 7;
        self.reg_v[reg_x] = self.reg_v[reg_x] << 1;
        debug_print(self.opcode);
    }

    // Skips the next instruction if VX does not equal VY
    fn op_0x9xy0(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.reg_v[reg_x] != self.reg_v[reg_y] {
            self.reg_pc += 2;
        }
        debug_print(self.opcode);
    }

    // Sets I to the address NNN.
    fn op_0xAnnn(&mut self) {
        self.reg_i = self.opcode & 0x0FFF;
        debug_print(self.opcode);
    }

    // Jumps to address NNN plus V0.
    fn op_0xBnnn(&mut self) {
        self.reg_pc = (self.opcode & 0x0FFF) + self.reg_v[0] as u16;
        debug_print(self.opcode);
    }

    // Sets Vx = random byte AND kk.
    fn op_0xCxkk(&mut self) {
        let mut rng = rand::thread_rng();
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_v[reg_x] = rng.gen_range(0..0xFF) & (self.opcode & 0x00FF) as u8;
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

    // Skips the next instruction if the key stored in VX is pressed.
    fn op_0xEx9E(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        if self.keyboard[self.reg_v[reg_x] as usize] == true {
            self.reg_pc += 2;
        }
        debug_print(self.opcode);
    }

    // Skips the next instruction if the key stored in VX is not pressed.
    fn op_0xExA1(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        if self.keyboard[self.reg_v[reg_x] as usize] == false {
            self.reg_pc += 2;
        }
        debug_print(self.opcode);
    }

    // Sets Vx = delay timer value.
    fn op_0xFx07(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_v[reg_x] = self.reg_delay_timer;
        debug_print(self.opcode);
    }

    // Awaits a key press, then stores the value of the key in VX.
    fn op_0xFx0A(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        let mut key_pressed = false;
        for i in 0..16 {
            if self.keyboard[i] == true {
                self.reg_v[reg_x] = i as u8;
                key_pressed = true;
            }
        }
        if !key_pressed {
            self.reg_pc -= 2;
        }
        debug_print(self.opcode);
    }

    // Sets the delay timer = Vx.
    fn op_0xFx15(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_delay_timer = self.reg_v[reg_x];
        debug_print(self.opcode);
    }

    // Sets the sound timer = Vx.
    fn op_0xFx18(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_sound_timer = self.reg_v[reg_x];
        debug_print(self.opcode);
    }

    // Adds Vx to I.
    fn op_0xFx1E(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_i += self.reg_v[reg_x] as u16;
        debug_print(self.opcode);
    }

    // Sets I = location of sprite for digit Vx.
    fn op_0xFx29(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.reg_i = self.reg_v[reg_x] as u16 * 5;
        debug_print(self.opcode);
    }

    // Stores BCD representation of Vx in memory locations I, I+1, and I+2.
    fn op_0xFx33(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.ram[self.reg_i as usize] = self.reg_v[reg_x] / 100;
        self.ram[self.reg_i as usize + 1] = (self.reg_v[reg_x] % 100) / 10;
        self.ram[self.reg_i as usize + 2] = self.reg_v[reg_x] % 10;
        debug_print(self.opcode);
    }

    // Stores registers V0 to Vx in memory starting at location I.
    fn op_0xFx55(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        for i in 0..reg_x + 1 {
            self.ram[self.reg_i as usize + i] = self.reg_v[i];
        }
        debug_print(self.opcode);
    }

    // Fills registers V0 to Vx with values from memory starting at location I.
    fn op_0xFx65(&mut self) {
        let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
        for i in 0..reg_x + 1 {
            self.reg_v[i] = self.ram[self.reg_i as usize + i];
        }
        debug_print(self.opcode);
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
