pub struct CPU {
  af: [u8; 2],
  bcdehl: [u8; 6],
  stack_pointer: [u8; 2],
  pc: u16,
  rom_bank: [u8; 0x7FFF + 1],
  vram: [u8; 0x1FFF + 1],
  external_ram: [u8; 0x1FFF + 1],
  wram: [u8; 0x1FFF + 1],
  oam: [u8; 160],
  hram: [u8; 127],
  io_ports: [u8; 96],
}

impl CPU {
  pub fn new() -> Self {
    CPU { af: [0x01, 0xB0], 
          bcdehl: [0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D], 
          pc: 0,
          stack_pointer: [0xFF, 0xFE],
          rom_bank: [0; 0x7FFF + 1],
          vram: [0; 0x1FFF + 1],
          external_ram: [0; 0x1FFF + 1],
          wram: [0; 0x1FFF + 1],
          oam: [0; 160],
          hram: [0; 127],
          io_ports: [0; 96],
        }
  }
  pub fn read(&self, addr: u16) -> u8 {
    match addr {
      0..=0x7FFF => { 
        self.rom_bank[addr as usize]
      },
      0x8000..=0x9FFF => { 
        let vram_addr: usize = (addr - 0x8000).into();
        self.vram[vram_addr]
      }, 
      0xA000..=0xBFFF => {
        let external_addr: usize = (addr - 0xA000).into();
        self.external_ram[external_addr]
      },
      0xC000..=0xDFFF => {
        let wram_addr: usize = (addr - 0xC000).into();
        self.wram[wram_addr]
      },
      0xFE00..=0xFE9F => {
        let oam_addr: usize = (addr - 0xC000).into();
        self.oam[oam_addr]
      },
      0xFF00..=0xFF4B => {
        let io_addr: usize = (addr - 0xFF00).into();
        self.io_ports[io_addr]
      },
      0xFF80..=0xFFF3 => {
        let hram_addr: usize = (addr - 0xFF80).into();
        self.hram[hram_addr]
      },
      _ => panic!("Non-covered memory"),
    }
  }

  pub fn write(&mut self, addr: u16, val: u8) {
    match addr {
      0..=0x7FFF => { 
        self.rom_bank[addr as usize] = val;
      },
      0x8000..=0x9FFF => { 
        let vram_addr: usize = (addr - 0x8000).into();
        self.vram[vram_addr] = val;
      }, 
      0xA000..=0xBFFF => {
        let external_addr: usize = (addr - 0xA000).into();
        self.external_ram[external_addr] = val;
      },
      0xC000..=0xDFFF => {
        let wram_addr: usize = (addr - 0xC000).into();
        self.wram[wram_addr] = val;
      },
      0xFE00..=0xFE9F => {
        let oam_addr: usize = (addr - 0xC000).into();
        self.oam[oam_addr] = val;
      },
      0xFF00..=0xFF4B => {
        let io_addr: usize = (addr - 0xFF00).into();
        self.io_ports[io_addr] = val;
      },
      0xFF80..=0xFFF3 => {
        let hram_addr: usize = (addr - 0xFF80).into();
        self.hram[hram_addr] = val;
      },
      _ => panic!("Non-covered memory"),
    }
  }

  pub fn fetch(&self) -> u8 {
     self.read(self.pc)
  }

  pub fn execute(&mut self, opcode: u8) {
    match opcode {
    0xC3 => { 
      self.pc = ((self.read(self.pc+1) as u16) << 8) + self.read(self.pc+2) as u16;
      println!("PC is now: 0x{:x}", self.pc);
    },
    _ => panic!("OPCode not supported"),

    }
  }
}
