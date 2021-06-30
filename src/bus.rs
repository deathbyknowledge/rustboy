pub struct Bus {
  ram: [u8; 64 * 1024],
}

#[allow(dead_code)]
impl Bus {
  pub fn new() -> Self {
    Bus { ram: [0; 64 * 1024] }
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.ram[addr as usize]
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.ram[addr as usize] = data;
  }
}

