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
