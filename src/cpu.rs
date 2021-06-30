use crate::bus::Bus;

#[allow(dead_code)]
pub struct CPU<'a> {
  // Registers
  af: [u8; 2],
  bcdehl: [u8; 6],
  sp: u16,
  pc: u16,

  // Data BUS
  bus: Option<&'a mut Bus>,
}

enum Flag {
  Z = 1 << 7, // Zero flag
  N = 1 << 6, // Substracion flag
  H = 1 << 5, // Half Carry flag
  C = 1 << 4, // Carry flag
}

#[allow(dead_code)]
impl<'a> CPU<'a> {

  pub fn new() -> Self {
    CPU { af: [0x01, 0xB0], 
          bcdehl: [0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D], 
          sp: 0xFFFE,
          pc: 0,
          bus: None,
        }
  }

  // FLAGS
  fn set_flag(&mut self, f: Flag, b: bool) {
    if b {
      self.af[1] |= f as u8;
    } else {
      self.af[1] &= !(f as u8);
    }
  }

  fn get_flag(&self, f: Flag) -> bool {
    match f {
      Flag::Z => ((self.af[1] >> 7) & 1) == 1,
      Flag::N => ((self.af[1] >> 6) & 1) == 1,
      Flag::H => ((self.af[1] >> 5) & 1) == 1,
      Flag::C => ((self.af[1] >> 4) & 1) == 1,
    }
  }

  // REGISTER GETTERS
  fn get_a(&self) -> u8 {
    self.af[0]
  }

  fn get_f(&self) -> u8 {
    self.af[1]
  }

  fn get_b(&self) -> u8 {
    self.bcdehl[0]
  }

  fn get_c(&self) -> u8 {
    self.bcdehl[1]
  }

  fn get_d(&self) -> u8 {
    self.bcdehl[2]
  }

  fn get_e(&self) -> u8 {
    self.bcdehl[3]
  }

  fn get_h(&self) -> u8 {
    self.bcdehl[4]
  }

  fn get_l(&self) -> u8 {
    self.bcdehl[5]
  }

  fn get_bc(&self) -> u16 {
    let hi = self.get_b() as u16;
    let lo = self.get_c() as u16;
    let bc = (hi << 8) + lo;
    bc
  }

  fn get_de(&self) -> u16 {
    let hi = self.get_d() as u16;
    let lo = self.get_e() as u16;
    let de = (hi << 8) + lo;
    de
  }

  fn get_hl(&self) -> u16 {
    let hi = self.get_h() as u16;
    let lo = self.get_l() as u16;
    let hl = (hi << 8) + lo;
    hl
  }

  fn get_sp(&self) -> u16 {
    self.sp
  }

  // REGISTER SETTERS
  fn set_a(&mut self, d: u8) {
    self.af[0] = d;
  }

  fn set_f(&mut self, d: u8) {
    self.af[1] = d;
  }

  fn set_b(&mut self, d: u8) {
    self.bcdehl[0] = d;
  }

  fn set_c(&mut self, d: u8) {
    self.bcdehl[1] = d;
  }

  fn set_d(&mut self, d: u8) {
    self.bcdehl[2] = d;
  }

  fn set_e(&mut self, d: u8) {
    self.bcdehl[3] = d;
  }

  fn set_h(&mut self, d: u8) {
    self.bcdehl[4] = d;
  }

  fn set_l(&mut self, d: u8) {
    self.bcdehl[5] = d;
  }
  
  fn set_bc(&mut self, d: u16) {
    let hi = (d >> 8) as u8;
    let lo = (d << 8) as u8;
    self.set_b(hi);
    self.set_c(lo);
  }
  
  fn set_de(&mut self, d: u16) {
    let hi = (d >> 8) as u8;
    let lo = (d << 8) as u8;
    self.set_d(hi);
    self.set_e(lo);
  }
  
  fn set_hl(&mut self, d: u16) {
    let hi = (d >> 8) as u8;
    let lo = (d << 8) as u8;
    self.set_h(hi);
    self.set_l(lo);
  }

  fn set_sp(&mut self, d: u16) {
    self.sp = d;
  }


  pub fn read(&self, a: u16) -> u8 {
    match &self.bus {
      Some(b) => b.read(a),
      None => panic!("No bus connected!"),
    }
  }

  pub fn write(&mut self, a: u16, d: u8) {
    match &mut self.bus {
      Some(b) => {
        b.write(a, d);
      },
      None => panic!("No bus connected!"),
    }
  }

  pub fn connect_bus(&mut self, bus: &'a mut Bus) {
    self.bus = Some(bus); 
  }

  pub fn fetch(&mut self) -> u8 {
    let opcode = self.read(self.pc);
    self.pc += 1;
    opcode
  }

  pub fn decode(&mut self, opcode: u8) -> (u8, fn(&mut CPU<'a>)) {
    match opcode {
      0x00 => (1, CPU::nop),
      0x01 => (3, CPU::ld_bc_nn),
      0x02 => (2, CPU::ld_mbc_a),
      0x03 => (2, CPU::inc_bc),
      0x04 => (1, CPU::inc_b),
      0x05 => (1, CPU::dec_b),
      0x06 => (2, CPU::ld_b_n),
      0x07 => (1, CPU::rlca),
      0x08 => (5, CPU::ld_nn_sp),
      0x09 => (2, CPU::add_hl_bc),
      0x0A => (2, CPU::ld_a_mbc),
      0x0B => (2, CPU::dec_bc),
      0x0C => (1, CPU::inc_c),
      0x0D => (1, CPU::dec_c),
      0x0E => (2, CPU::ld_c_n),
      0x0F => (1, CPU::rrca),
      0x10 => (1, CPU::stop),
      0x11 => (3, CPU::ld_de_nn),
      0x12 => (2, CPU::ld_mde_a),
      0x13 => (2, CPU::inc_de),
      0x14 => (1, CPU::inc_d),
      0x15 => (1, CPU::dec_d),
      0x16 => (2, CPU::ld_d_n),
      0x17 => (1, CPU::rla),
      0x18 => (5, CPU::jr_n),
      0x19 => (2, CPU::add_hl_de),
      0x1A => (2, CPU::ld_a_mde),
      0x1B => (2, CPU::dec_de),
      0x1C => (1, CPU::inc_e),
      0x1D => (1, CPU::dec_e),
      0x1E => (2, CPU::ld_e_n),
      0x1F => (1, CPU::rra),
      0x20 => (1, CPU::jr_nz_n), // here
      0x21 => (3, CPU::ld_de_nn),
      0x22 => (2, CPU::ld_mde_a),
      0x23 => (2, CPU::inc_de),
      0x24 => (1, CPU::inc_d),
      0x25 => (1, CPU::dec_d),
      0x26 => (2, CPU::ld_d_n),
      0x27 => (1, CPU::rla),
      0x28 => (5, CPU::jr_n),
      0x29 => (2, CPU::add_hl_de),
      0x2A => (2, CPU::ld_a_mde),
      0x2B => (2, CPU::dec_de),
      0x2C => (1, CPU::inc_e),
      0x2D => (1, CPU::dec_e),
      0x2E => (2, CPU::ld_e_n),
      0x2F => (2, CPU::ld_e_n),
      _ => (1, CPU::nop),
    }
  }


  /*
  -------------------
    INSTRUCTION SET
  -------------------
  */

  fn nop(&mut self) {}

  fn ld_bc_nn(&mut self) {
    self.set_c(self.read(self.pc));
    self.pc += 1;
    self.set_b(self.read(self.pc));
    self.pc += 1;
  }

  fn ld_mbc_a(&mut self) {
    self.write(self.get_bc(), self.get_a());
  }

  fn inc_bc(&mut self) {
    self.set_bc(self.get_bc() + 1);
  }

  fn inc_b(&mut self) {
    let is_half_carry = (((self.get_b() & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = self.get_b() + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_b(&mut self) {
    let is_half_carry = (((self.get_b() & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = self.get_b() - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_b_n(&mut self) {
    self.set_b(self.read(self.pc));
    self.pc += 1;
  }

  fn rlca(&mut self) {
    // http://jgmalcolm.com/z80/advanced/shif
    let is_carry = self.get_a() > 0x7F;
    self.set_flag(Flag::C, is_carry); 
    self.set_flag(Flag::Z, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, false);
    let mut res = self.get_a() << 1;
    if is_carry { res += 1; }
    self.set_a(res);
  }

  fn ld_nn_sp(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.pc += 1;
    let nn = (hi << 8) + lo;
    let lo = lsb(self.sp);
    let hi = msb(self.sp);
    self.write(nn, lo);
    self.write(nn + 1, hi);
  }

  fn add_hl_bc(&mut self) {
    let res = self.get_hl() + self.get_bc(); 
    let is_half_carry = (((self.get_hl() & 0xFFF) + (self.get_bc() & 0xFFF)) & 0x1000) != 0; 
    self.set_flag(Flag::C, res > 0xFFFF);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
  }

  fn ld_a_mbc(&mut self) {
    self.set_a(self.read(self.get_bc()));
  }

  fn dec_bc(&mut self) {
    self.set_bc(self.get_bc() - 1);
  }

  fn inc_c(&mut self) {
    let is_half_carry = (((self.get_c() & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = self.get_c() + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_c(&mut self) {
    let is_half_carry = (((self.get_c() & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = self.get_c() - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_c_n(&mut self) {
    self.set_c(self.read(self.pc));
    self.pc += 1;
  }

  fn rrca(&mut self) {
    let is_carry = self.get_a() % 2 != 0;
    self.set_flag(Flag::C, is_carry); 
    self.set_flag(Flag::Z, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, false);
    let mut res = self.get_a() >> 1;
    if is_carry { res += 0x10; }
    self.set_a(res);
  }

  fn stop(&mut self) {
    self.pc += 1; 
  }

  fn ld_de_nn(&mut self) {
    self.set_e(self.read(self.pc));
    self.pc += 1;
    self.set_d(self.read(self.pc));
    self.pc += 1;
  }

  fn ld_mde_a(&mut self) {
    self.write(self.get_de(), self.get_a());
  }

  fn inc_de(&mut self) {
    self.set_de(self.get_de() + 1);
  }

  fn inc_d(&mut self) {
    let is_half_carry = (((self.get_d() & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = self.get_d() + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_d(&mut self) {
    let is_half_carry = (((self.get_d() & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = self.get_d() - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_d_n(&mut self) {
    self.set_b(self.read(self.pc));
    self.pc += 1;
  }

  fn rla(&mut self) {
    let is_carry = self.get_a() > 0x7F;
    let first_bit = self.get_flag(Flag::C);
    self.set_flag(Flag::C, is_carry); 
    self.set_flag(Flag::Z, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, false);
    let mut res = self.get_a() << 1;
    if first_bit { res += 1; }
    self.set_a(res);
  }

  fn jr_n(&mut self) {
    let signed_int = self.read(self.pc);
    if signed_int > 127 {
      self.pc += (signed_int & 0x7F) as u16;
    } else {
      self.pc -= (signed_int & 0x7F) as u16;
    }
  }

  fn add_hl_de(&mut self) {
    let res = self.get_hl() + self.get_de(); 
    let is_half_carry = (((self.get_hl() & 0xFFF) + (self.get_de() & 0xFFF)) & 0x1000) != 0; 
    self.set_flag(Flag::C, res > 0xFFFF);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
  }

  fn ld_a_mde(&mut self) {
    self.set_a(self.read(self.get_de()));
  }

  fn dec_de(&mut self) {
    self.set_bc(self.get_de() - 1);
  }

  fn inc_e(&mut self) {
    let is_half_carry = (((self.get_e() & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = self.get_e() + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_e(&mut self) {
    let is_half_carry = (((self.get_e() & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = self.get_e() - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_e_n(&mut self) {
    self.set_e(self.read(self.pc));
    self.pc += 1;
  }

  fn rra(&mut self) {
    let is_carry = self.get_a() % 2 != 0;
    let last_bit = self.get_flag(Flag::C);
    self.set_flag(Flag::C, is_carry); 
    self.set_flag(Flag::Z, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, false);
    let mut res = self.get_a() >> 1;
    if last_bit { res += 0x10; }
    self.set_a(res);
  }
 
  fn jr_nz_n(&mut self) {
    let signed_int = self.read(self.pc);
    self.pc += 1;
    if self.get_flag(Flag::N) && self.get_flag(Flag::Z) {
      if signed_int > 127 {
        self.pc += (signed_int & 0x7F) as u16;
      } else {
        self.pc -= (signed_int & 0x7F) as u16;
      }
    }
  }

  fn ld_hl_nn(&mut self) {
    self.set_l(self.read(self.pc));
    self.pc += 1;
    self.set_h(self.read(self.pc));
    self.pc += 1;
  }

  fn ldi_mhl_a(&mut self) {
    self.write(self.get_hl(), self.get_a());
    self.set_hl(self.get_hl() + 1);
  }

  fn inc_hl(&mut self) {
    self.set_hl(self.get_hl() + 1);
  }

  fn inc_h(&mut self) {
    let is_half_carry = (((self.get_h() & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = self.get_h() + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_h(&mut self) {
    let is_half_carry = (((self.get_h() & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = self.get_h() - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_h_n(&mut self) {
    self.set_h(self.read(self.pc));
    self.pc += 1;
  }

  fn daa(&mut self) {
  // https://forums.nesdev.com/viewtopic.php?t=15944
  // after an addition, adjust if (half-)carry occurred or if result is out of bounds
    if !self.get_flag(Flag::N) {
      if self.get_flag(Flag::C) || self.get_a() > 0x99 {
        self.set_a(self.get_a() + 0x60);  
        self.set_flag(Flag::C, true);
      }
      if self.get_flag(Flag::H) || (self.get_a() & 0x0F) > 0x09 {
        self.set_a(self.get_a() + 0x6); 
      }
  // after a subtraction, only adjust if (half-)carry occurred
    } else {
      if self.get_flag(Flag::C) { self.set_a(self.get_a() - 0x60); }
      if self.get_flag(Flag::H) { self.set_a(self.get_a() - 0x6); }
    }
  }

  fn jr_z_n(&mut self) {
    let signed_int = self.read(self.pc);
    self.pc += 1;
    if self.get_flag(Flag::Z) {
      if signed_int > 127 {
        self.pc += (signed_int & 0x7F) as u16;
      } else {
        self.pc -= (signed_int & 0x7F) as u16;
      }
    }
  }

  fn add_hl_hl(&mut self) {
    let res = self.get_hl() + self.get_hl(); 
    let is_half_carry = (((self.get_hl() & 0xFFF) + (self.get_hl() & 0xFFF)) & 0x1000) != 0; 
    self.set_flag(Flag::C, res > 0xFFFF);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
  }

  fn ldi_a_mhl(&mut self) {
    self.set_a(self.read(self.get_hl()));
    self.set_hl(self.get_hl() + 1);
  }

  fn dec_hl(&mut self) {
    self.set_hl(self.get_hl() - 1);
  }

  fn inc_l(&mut self) {
    let is_half_carry = (((self.get_l() & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = self.get_l() + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_l(&mut self) {
    let is_half_carry = (((self.get_l() & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = self.get_l() - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_l_n(&mut self) {
    self.set_l(self.read(self.pc));
    self.pc += 1;
  }

  fn cpl(&mut self) {
    self.set_a(!self.get_a());
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::H, true);
  }
}

fn msb(d: u16) -> u8 {
  (d >> 8) as u8
}

fn lsb(d: u16) -> u8 {
  (d & 0xFF) as u8
}
