use std::{fs::File, io::Read};
pub struct Disk {
    pub rom: [u8; 4095],
    pub size: usize,
}

impl Disk {
    pub fn new(file_path: &str) -> Disk {
        let mut rom = [0; 4095];
        let mut file = File::open(file_path).unwrap();
        let size = file.read(&mut rom).unwrap();
        Disk {
            rom: rom,
            size: size,
        }
    }

    pub fn print_disk(&self) {
        for i in 0..self.size {
            print!("{:02x} ", self.rom[i as usize]);
            if i % 16 == 15 {
                println!("");
            }
        }
        println!("");
    }
}
