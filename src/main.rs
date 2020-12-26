mod cpu;
mod mmu;
mod console;
mod cartridge;
mod ppu;
mod io;
mod operations;
mod dma;
mod timer;
mod screen;
mod logger;
mod joypad;

extern crate minifb;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

use crate::console::{Console};

fn main() {
    let mut console: Console = Console::new();
    let cart_path = "./roms/cpu_instrs.gb";

    console.load(cart_path);
    console.reset();
    console.execute_ticks(45165847);

    console.execute_ticks(1);
    print!("finished")
}
