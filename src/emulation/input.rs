use piston_window::Key;

use super::Cpu;

pub fn handle_input(cpu: &mut Cpu, key: Key) {
    //println!("{:?}", key);
    let input = match key {
        Key::D1 => 0x1,
        Key::D2 => 0x2,
        Key::D3 => 0x3,
        Key::D4 => 0xc,
        Key::Q => 0x4,
        Key::W => 0x5,
        Key::E => 0x6,
        Key::R => 0xd,
        Key::A => 0x7,
        Key::S => 0x8,
        Key::D => 0x9,
        Key::F => 0xe,
        Key::Y => 0xa,
        Key::Z => 0xa,
        Key::X => 0x0,
        Key::C => 0xb,
        Key::V => 0xf,
        _ => 0x0,
    };

    if input != 0x0 {
        cpu.key_pressed(input);
    }
}
