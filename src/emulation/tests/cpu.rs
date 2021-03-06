#[cfg(test)]

const Rom_Dummy: [u8; 3] = [0x42; 3];

mod tests {
    use super::*;
    use crate::emulation::{Cpu, Disk};

    // Cpu instantiation
    #[test]
    fn cpu_new() {
        let cpu = Cpu::new();
        assert_eq!(cpu.reg_v[0], 0);
        assert_eq!(cpu.ram[0], 0xF0); // 0xF0 is the value of the first font character
    }

    // Load disk to ram
    #[test]
    fn cpu_load_disk_to_ram() {
        // Disk load stub
        let disk = disk_load_stub(&Rom_Dummy);
        let mut cpu = Cpu::new();

        cpu.load_disk_to_ram(&disk);

        assert_eq!(cpu.ram[0], 0xF0); // 0xF0 is the value of the first font character

        for i in 0..Rom_Dummy.len() {
            assert_eq!(cpu.ram[0x200 + i], 0x42);
        }
    }

    // Next Cpu tick
    #[test]
    fn cpu_next() {
        assert_eq!(1, 1);
        //TODO: Add tests after next is completed
    }

    // Test Opcode 0x00E0
    #[test]
    fn cpu_0x00E0() {
        let mut cpu = get_cpu_with_opcode(0x00E0);

        // Dirty up the video ram
        assert_eq!(cpu.video_ram[0][0], 0);
        cpu.video_ram[0][0] = 1;
        assert_eq!(cpu.video_ram[0][0], 1);

        // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.video_ram[0][0], 0);
    }

    // Test Opcode 0x00EE
    #[test]
    fn cpu_0x00EE() {
        let mut cpu = get_cpu_with_opcode(0x00EE);

        // Put dummy addr on stack
        cpu.stack[0] = 1;
        cpu.reg_sp = 1;

        // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.stack_ptr, 0);
    }

    // Test Opcode 0x1NNN
    #[test]
    fn cpu_0x1NNN() {
        let mut cpu = get_cpu_with_opcode(0x1123);

        // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x123);
    }

    // Test Opcode 0x2NNN
    #[test]
    fn cpu_0x2NNN() {
        let mut cpu = get_cpu_with_opcode(0x2123);

        // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x123);
    }

    // Test Opcode 0x3XNN
    #[test]
    fn cpu_0x3XNN() {
        let mut cpu = get_cpu_with_opcode(0x3101);

        // False case
        cpu.reg_v[1] = 0;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x200 + 0);

        // True case
        cpu.reg_v[1] = 1;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x200 + 2);
    }

    // Test Opcode 0x4XNN
    #[test]
    fn cpu_0x4XNN() {
        let mut cpu = get_cpu_with_opcode(0x4101);

        // False case
        cpu.reg_v[1] = 1;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x200 + 0);

        // True case
        cpu.reg_v[1] = 0;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x200 + 2);
    }

    // Test Opcode 0x5XY0
    #[test]
    fn cpu_0x5XY0() {
        let mut cpu = get_cpu_with_opcode(0x5120);

        // False case
        cpu.reg_v[1] = 0;
        cpu.reg_v[2] = 1;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x200 + 0);

        // True case
        cpu.reg_v[1] = 1;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x200 + 2);
    }

    // Test Opcode 0x6XNN
    #[test]
    fn cpu_0x6XNN() {
        let mut cpu = get_cpu_with_opcode(0x6101);

        // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0x01);
    }

    // Test Opcode 0x7XNN
    #[test]
    fn cpu_0x7XNN() {
        let mut cpu = get_cpu_with_opcode(0x7101);
        cpu.reg_v[1] = 1;

        // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0x02);
    }

    // Test Opcode 0x8XY0
    #[test]
    fn cpu_0x8XY0() {
        let mut cpu = get_cpu_with_opcode(0x8120);
        cpu.reg_v[1] = 1; // x
        cpu.reg_v[2] = 2; // y

        // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 2);
    }

    // Test Opcode 0x8XY1
    #[test]
    fn cpu_0x8XY1() {
        let mut cpu = get_cpu_with_opcode(0x8121);
        cpu.reg_v[1] = 1; // x
        cpu.reg_v[2] = 2; // y
                          // 0 1
                          // 1 0
                          // 1 1
                          // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 3);
    }

    // Test Opcode 0x8XY2
    #[test]
    fn cpu_0x8XY2() {
        let mut cpu = get_cpu_with_opcode(0x8122);
        cpu.reg_v[1] = 1; // x
        cpu.reg_v[2] = 3; // y
                          // 0 1
                          // 1 1
                          // 0 1
                          // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 1);
    }

    // Test Opcode 0x8XY3
    #[test]
    fn cpu_0x8XY3() {
        let mut cpu = get_cpu_with_opcode(0x8123);
        cpu.reg_v[1] = 1; // x
        cpu.reg_v[2] = 3; // y
                          // 0 1
                          // 1 1
                          // 1 0
                          // Execute the opcode
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 2);
    }

    // Test Opcode 0x8XY4
    #[test]
    fn cpu_0x8XY4() {
        let mut cpu = get_cpu_with_opcode(0x8124);

        // No overflow
        cpu.reg_v[1] = 1; // vx
        cpu.reg_v[2] = 3; // vy
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 4);
        assert_eq!(cpu.reg_v[15], 0);

        // Overflow
        cpu.reg_v[1] = 0xFF; // vx
        cpu.reg_v[2] = 1; // vy
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0);
        assert_eq!(cpu.reg_v[15], 1);
    }

    // Test Opcode 0x8XY5
    #[test]
    fn cpu_0x8XY5() {
        let mut cpu = get_cpu_with_opcode(0x8125);

        // No borrow
        cpu.reg_v[1] = 1; // vx
        cpu.reg_v[2] = 2; // vy
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0xFF);
        assert_eq!(cpu.reg_v[15], 0);

        // Borrow
        cpu.reg_v[1] = 2; // vx
        cpu.reg_v[2] = 1; // vy
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 1);
        assert_eq!(cpu.reg_v[15], 1);
    }

    // Test Opcode 0x8XY6
    #[test]
    fn cpu_0x8XY6() {
        let mut cpu = get_cpu_with_opcode(0x8106);

        // Even LSB
        cpu.reg_v[1] = 4; // vx
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 2);
        assert_eq!(cpu.reg_v[15], 0);

        // Uneven LSB
        cpu.reg_v[1] = 5; // vx
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 2);
        assert_eq!(cpu.reg_v[15], 1);
    }

    // Test Opcode 0x8XY7
    #[test]
    fn cpu_0x8XY7() {
        let mut cpu = get_cpu_with_opcode(0x8127);
        // Vx = Vy - Vx
        cpu.reg_v[1] = 1; // vx
        cpu.reg_v[2] = 0x0F; // vy
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0x0E);
        assert_eq!(cpu.reg_v[15], 1);

        cpu.reg_v[1] = 0xFF; // vx
        cpu.reg_v[2] = 0x0F; // vy
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0x10);
        assert_eq!(cpu.reg_v[15], 0);
    }

    // Test Opcode 0x8XYE
    #[test]
    fn cpu_0x8XYE() {
        let mut cpu = get_cpu_with_opcode(0x812E);

        cpu.reg_v[1] = 0xC0; // vx
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0x80);
        assert_eq!(cpu.reg_v[15], 1);
    }

    // Test Opcode 0x9XY0
    #[test]
    fn cpu_0x9xy0() {
        let mut cpu = get_cpu_with_opcode(0x9120);

        // Jump
        cpu.reg_pc = 0;
        cpu.reg_v[1] = 1; // vx
        cpu.reg_v[2] = 2; // vy
        cpu.execute();
        assert_eq!(cpu.reg_pc, 2);

        // Don't jump
        cpu.reg_pc = 0;
        cpu.reg_v[1] = 1; // vx
        cpu.reg_v[2] = 1; // vy
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0);
    }

    // Test Opcode 0xANNN
    #[test]
    fn cpu_0xAnnn() {
        let mut cpu = get_cpu_with_opcode(0xA123);
        cpu.execute();
        assert_eq!(cpu.reg_i, 0x123);
    }

    // Test Opcode 0xBNNN
    #[test]
    fn cpu_0xBnnn() {
        let mut cpu = get_cpu_with_opcode(0xB123);
        cpu.reg_v[0] = 1; // vx
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0x124);
    }

    // Test Opcode 0xCXNN
    #[test]
    fn cpu_0xCxnn() {
        let mut cpu = get_cpu_with_opcode(0xC1FF);
        cpu.execute();
        println!("Random Value {:X}", cpu.reg_v[1]);
    }

    // Test Opcode 0xDXYN
    #[test]
    fn cpu_0xDxyn() {
        println!("No test case avaialble for Opcode 0xDXYN");
    }

    // Test Opcode 0xEX9E
    #[test]
    fn cpu_0xEx9e() {
        let mut cpu = get_cpu_with_opcode(0xE19E);
        cpu.reg_v[1] = 1; // vx
        cpu.keyboard[1] = true; // key

        // Pressed
        cpu.reg_pc = 0;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 2);

        // Not pressed
        cpu.reg_pc = 0;
        cpu.keyboard[1] = false;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0);
    }

    // Test Opcode 0xEXA1
    #[test]
    fn cpu_0xExa1() {
        let mut cpu = get_cpu_with_opcode(0xE1A1);
        cpu.reg_v[1] = 1; // vx
        cpu.keyboard[1] = true; // key

        // Pressed
        cpu.reg_pc = 0;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0);

        // Not pressed
        cpu.reg_pc = 0;
        cpu.keyboard[1] = false;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 2);
    }

    // Test Opcode 0xFX07
    #[test]
    fn cpu_0xFx07() {
        let mut cpu = get_cpu_with_opcode(0xF107);
        cpu.reg_delay_timer = 0x20;
        cpu.execute();
        assert_eq!(cpu.reg_v[1], 0x20);
    }

    // Test Opcode 0xFX0A
    #[test]
    fn cpu_0xFx0a() {
        let mut cpu = get_cpu_with_opcode(0xF10A);

        // Not pressed -> PC -= 2
        cpu.reg_pc = 2;
        cpu.reg_v[1] = 1; // vx
        cpu.keyboard[1] = false; // key
        cpu.execute();
        assert_eq!(cpu.reg_pc, 0);

        // Pressed -> PC == 2
        cpu.keyboard[1] = true; // key
        cpu.reg_pc = 2;
        cpu.execute();
        assert_eq!(cpu.reg_pc, 2);
    }

    // Test Opcode 0xFX15
    #[test]
    fn cpu_0xFx15() {
        let mut cpu = get_cpu_with_opcode(0xF115);
        cpu.reg_delay_timer = 0;
        cpu.reg_v[1] = 0x20; // vx
        cpu.execute();
        assert_eq!(cpu.reg_delay_timer, 0x20);
    }

    // Test Opcode 0xFX18
    #[test]
    fn cpu_0xFx18() {
        let mut cpu = get_cpu_with_opcode(0xF118);
        cpu.reg_sound_timer = 0;
        cpu.reg_v[1] = 0x20; // vx
        cpu.execute();
        assert_eq!(cpu.reg_sound_timer, 0x20);
    }

    // Test Opcode 0xFX1E
    #[test]
    fn cpu_0xFx1e() {
        let mut cpu = get_cpu_with_opcode(0xF11E);
        cpu.reg_i = 0x123;
        cpu.reg_v[1] = 0x20; // vx
        cpu.execute();
        assert_eq!(cpu.reg_i, 0x123 + 0x20);
    }

    // Test Opcode 0xFX29
    #[test]
    fn cpu_0xFx29() {
        let mut cpu = get_cpu_with_opcode(0xF129);
        cpu.reg_v[1] = 1; // vx
        cpu.execute();
        assert_eq!(cpu.reg_i, 5);
    }

    // Test Opcode 0xFX33
    #[test]
    fn cpu_0xFx33() {
        let mut cpu = get_cpu_with_opcode(0xF133);
        cpu.reg_v[1] = 123; // vx
        cpu.execute();
        assert_eq!(cpu.ram[cpu.reg_i as usize], 1);
        assert_eq!(cpu.ram[cpu.reg_i as usize + 1], 2);
        assert_eq!(cpu.ram[cpu.reg_i as usize + 2], 3);
    }

    // Test Opcode 0xFX55
    #[test]
    fn cpu_0xFx55() {
        let mut cpu = get_cpu_with_opcode(0xF255);
        cpu.reg_v[0] = 0x12; // vx
        cpu.reg_v[1] = 0x34; // vy
        cpu.reg_v[2] = 0x56; // vz
        cpu.execute();
        assert_eq!(cpu.ram[cpu.reg_i as usize], 0x12);
        assert_eq!(cpu.ram[cpu.reg_i as usize + 1], 0x34);
        assert_eq!(cpu.ram[cpu.reg_i as usize + 2], 0x56);
    }

    // Test Opcode 0xFX65
    #[test]
    fn cpu_0xFx65() {
        let mut cpu = get_cpu_with_opcode(0xF265);
        cpu.reg_v[0] = 0; // vx
        cpu.reg_v[1] = 0; // vy
        cpu.reg_v[2] = 0; // vz
        cpu.ram[cpu.reg_i as usize] = 0x12;
        cpu.ram[cpu.reg_i as usize + 1] = 0x34;
        cpu.ram[cpu.reg_i as usize + 2] = 0x56;
        cpu.execute();
        assert_eq!(cpu.reg_v[0], 0x12);
        assert_eq!(cpu.reg_v[1], 0x34);
        assert_eq!(cpu.reg_v[2], 0x56);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // HELPER FUNCTIONS
    ////////////////////////////////////////////////////////////////////////////////

    // Opcode test helper
    fn get_cpu_with_opcode(opcode: u16) -> Cpu {
        let mut cpu = Cpu::new();
        cpu.opcode = opcode;
        return cpu;
    }

    // Disk load stub
    fn disk_load_stub(rom_array: &[u8]) -> Disk {
        let mut disk = Disk {
            rom: [0; 4095],
            size: rom_array.len(),
        };
        for i in 0..rom_array.len() {
            disk.rom[i] = rom_array[i];
        }
        return disk;
    }
}
