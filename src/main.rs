use std::env::args;
use std::io::prelude::*;
use std::fs::File;

mod display;
mod cpu;
use display::Display;
use cpu::CPU;

struct Gameboy {
 cpu: CPU,
 wram: [u8; 8196],
 vram: [u8; 8196],
 display: Display,
 game: Vec<u8>,
}

impl Gameboy {
  fn new() -> Self {
    Gameboy { 
      cpu: CPU::new(),
      wram: [0; 8196],
      vram: [0; 8196],
      display: Display::new(),
      game: Vec::new(),
    }
  }

  fn boot_game(&mut self) {
    // Compare NINTENDO LOGO to logo range in the ROM header. If they're not the same, panic.
    if &self.game[0x104..0x134] == NINTENDO_LOGO {

      // Check range 0x104-0x133 and compute checksum by adding all bytes together
      // + 25 in decimal. If the least significant bit is NOT 0, panic.
      let mut sum: u16 = 25;
      for byte in &self.game[0x134..0x14d] {
        sum += *byte as u16; 
      }
      if sum % 2 != 0 {
        panic!("Invalid checksum");
      }
    } else {
      panic!("Invalid Nintento logo");
    }

    // Since it's a valid game, update the window's name to the Game's name
    let game_title = &String::from_utf8(self.game[0x134..0x144].to_vec()).unwrap();
    self.display.set_title(game_title);

    //self.cpu.MEM[0..0x8000] = self.game[..];
  }

  fn start(&mut self) {
    while let Some(e) = self.display.poll() {
      let opcode = self.cpu.fetch();
      self.cpu.execute(opcode);
   		self.display.refresh(&e); 
    }
  }
}

fn main() {
  let mut gb = Gameboy::new();
  let file_name = args().nth(1).unwrap();
  let mut f = File::open(&file_name).unwrap();
  f.read_to_end(&mut gb.game).unwrap();

  gb.boot_game();
  gb.start();
}

const NINTENDO_LOGO: &[u8] = &[206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0, 8, 17, 31, 136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187, 103, 99, 110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62];
