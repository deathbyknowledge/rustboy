use crate::bus::Bus;

pub struct CPU<'a> {
 // Registers
  af: [u8; 2],
  bcdehl: [u8; 6],
  sp: u16,
  pc: u16,

  ime: bool,

  // Data BUS
  bus: Option<&'a mut Bus>,
}

enum Flag {
  Z = 1 << 7, // Zero flag
  N = 1 << 6, // Substracion flag
  H = 1 << 5, // Half Carry flag
  C = 1 << 4, // Carry flag
}

impl<'a> CPU<'a> {

  pub fn new() -> Self {
    CPU { af: [0x01, 0xB0], 
          bcdehl: [0x00, 0x13, 0x00, 0xD8, 0x01, 0x4D], 
          sp: 0xFFFE,
          pc: 0,
          bus: None,
          ime: false,
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

  fn set_af(&mut self, d: u16) {
    let hi = (d >> 8) as u8;
    let lo = (d << 8) as u8;
    self.set_a(hi);
    self.set_f(lo);
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
      0x18 => (4, CPU::jr_n),
      0x19 => (2, CPU::add_hl_de),
      0x1A => (2, CPU::ld_a_mde),
      0x1B => (2, CPU::dec_de),
      0x1C => (1, CPU::inc_e),
      0x1D => (1, CPU::dec_e),
      0x1E => (2, CPU::ld_e_n),
      0x1F => (1, CPU::rra),
      0x20 => (3, CPU::jr_nz_n), //TBD: 12/8
      0x21 => (3, CPU::ld_hl_nn),
      0x22 => (2, CPU::ldi_mhl_a),
      0x23 => (2, CPU::inc_hl),
      0x24 => (1, CPU::inc_h),
      0x25 => (1, CPU::dec_h),
      0x26 => (2, CPU::ld_h_n),
      0x27 => (1, CPU::daa),
      0x28 => (3, CPU::jr_z_n), // TBD: 12/8
      0x29 => (2, CPU::add_hl_hl),
      0x2A => (2, CPU::ldi_a_mhl),
      0x2B => (2, CPU::dec_hl),
      0x2C => (1, CPU::inc_l),
      0x2D => (1, CPU::dec_l),
      0x2E => (2, CPU::ld_l_n),
      0x2F => (1, CPU::cpl),
      0x30 => (3, CPU::jr_nc_n), //TBD: 12/8
      0x31 => (3, CPU::ld_sp_nn),
      0x32 => (2, CPU::ldd_mhl_a),
      0x33 => (2, CPU::inc_sp),
      0x34 => (3, CPU::inc_mhl),
      0x35 => (3, CPU::dec_mhl),
      0x36 => (3, CPU::ld_mhl_n),
      0x37 => (1, CPU::scf),
      0x38 => (3, CPU::jr_c_n), //TBD: 12/8
      0x39 => (2, CPU::add_hl_sp),
      0x3A => (2, CPU::ldd_a_mhl),
      0x3B => (2, CPU::dec_sp),
      0x3C => (1, CPU::inc_a),
      0x3D => (1, CPU::dec_a),
      0x3E => (2, CPU::ld_a_n),
      0x3F => (1, CPU::ccf),
      0x40 => (1, CPU::ld_b_b),
      0x41 => (1, CPU::ld_b_c),
      0x42 => (1, CPU::ld_b_d),
      0x43 => (1, CPU::ld_b_e),
      0x44 => (1, CPU::ld_b_h),
      0x45 => (1, CPU::ld_b_l),
      0x46 => (2, CPU::ld_b_mhl),
      0x47 => (1, CPU::ld_b_a),
      0x48 => (1, CPU::ld_c_b),
      0x49 => (1, CPU::ld_c_c),
      0x4A => (1, CPU::ld_c_d),
      0x4B => (1, CPU::ld_c_e),
      0x4C => (1, CPU::ld_c_h),
      0x4D => (1, CPU::ld_c_l),
      0x4E => (2, CPU::ld_c_mhl),
      0x4F => (1, CPU::ld_c_a),
      0x50 => (1, CPU::ld_d_b),
      0x51 => (1, CPU::ld_d_c),
      0x52 => (1, CPU::ld_d_d),
      0x53 => (1, CPU::ld_d_e),
      0x54 => (1, CPU::ld_d_h),
      0x55 => (1, CPU::ld_d_l),
      0x56 => (2, CPU::ld_d_mhl),
      0x57 => (1, CPU::ld_d_a),
      0x58 => (1, CPU::ld_e_b),
      0x59 => (1, CPU::ld_e_c),
      0x5A => (1, CPU::ld_e_d),
      0x5B => (1, CPU::ld_e_e),
      0x5C => (1, CPU::ld_e_h),
      0x5D => (1, CPU::ld_e_l),
      0x5E => (2, CPU::ld_e_mhl),
      0x5F => (1, CPU::ld_e_a),
      0x60 => (1, CPU::ld_h_b),
      0x61 => (1, CPU::ld_h_c),
      0x62 => (1, CPU::ld_h_d),
      0x63 => (1, CPU::ld_h_e),
      0x64 => (1, CPU::ld_h_h),
      0x65 => (1, CPU::ld_h_l),
      0x66 => (2, CPU::ld_h_mhl),
      0x67 => (1, CPU::ld_h_a),
      0x68 => (1, CPU::ld_l_b),
      0x69 => (1, CPU::ld_l_c),
      0x6A => (1, CPU::ld_l_d),
      0x6B => (1, CPU::ld_l_e),
      0x6C => (1, CPU::ld_l_h),
      0x6D => (1, CPU::ld_l_l),
      0x6E => (2, CPU::ld_l_mhl),
      0x6F => (1, CPU::ld_l_a),
      0x70 => (2, CPU::ld_mhl_b),
      0x71 => (2, CPU::ld_mhl_c),
      0x72 => (2, CPU::ld_mhl_d),
      0x73 => (2, CPU::ld_mhl_e),
      0x74 => (2, CPU::ld_mhl_h),
      0x75 => (2, CPU::ld_mhl_l),
      0x76 => (1, CPU::halt),
      0x77 => (1, CPU::ld_mhl_a),
      0x78 => (1, CPU::ld_a_b),
      0x79 => (1, CPU::ld_a_c),
      0x7A => (1, CPU::ld_a_d),
      0x7B => (1, CPU::ld_a_e),
      0x7C => (1, CPU::ld_a_h),
      0x7D => (1, CPU::ld_a_l),
      0x7E => (2, CPU::ld_a_mhl),
      0x7F => (1, CPU::ld_a_a),
      0x80 => (1, CPU::add_a_b),
      0x81 => (1, CPU::add_a_c),
      0x82 => (1, CPU::add_a_d),
      0x83 => (1, CPU::add_a_e),
      0x84 => (1, CPU::add_a_h),
      0x85 => (1, CPU::add_a_l),
      0x86 => (2, CPU::add_a_mhl),
      0x87 => (1, CPU::add_a_a),
      0x88 => (1, CPU::adc_a_b),
      0x89 => (1, CPU::adc_a_c),
      0x8A => (1, CPU::adc_a_d),
      0x8B => (1, CPU::adc_a_e),
      0x8C => (1, CPU::adc_a_h),
      0x8D => (1, CPU::adc_a_l),
      0x8E => (2, CPU::adc_a_mhl),
      0x8F => (1, CPU::adc_a_a),
      0x90 => (1, CPU::sub_a_b),
      0x91 => (1, CPU::sub_a_c),
      0x92 => (1, CPU::sub_a_d),
      0x93 => (1, CPU::sub_a_e),
      0x94 => (1, CPU::sub_a_h),
      0x95 => (1, CPU::sub_a_l),
      0x96 => (2, CPU::sub_a_mhl),
      0x97 => (1, CPU::sub_a_a),
      0x98 => (1, CPU::sbc_a_b),
      0x99 => (1, CPU::sbc_a_c),
      0x9A => (1, CPU::sbc_a_d),
      0x9B => (1, CPU::sbc_a_e),
      0x9C => (1, CPU::sbc_a_h),
      0x9D => (1, CPU::sbc_a_l),
      0x9E => (2, CPU::sbc_a_mhl),
      0x9F => (1, CPU::sbc_a_a),
      0xA0 => (1, CPU::and_a_b),
      0xA1 => (1, CPU::and_a_c),
      0xA2 => (1, CPU::and_a_d),
      0xA3 => (1, CPU::and_a_e),
      0xA4 => (1, CPU::and_a_h),
      0xA5 => (1, CPU::and_a_l),
      0xA6 => (2, CPU::and_a_mhl),
      0xA7 => (1, CPU::and_a_a),
      0xA8 => (1, CPU::xor_a_b),
      0xA9 => (1, CPU::xor_a_c),
      0xAA => (1, CPU::xor_a_d),
      0xAB => (1, CPU::xor_a_e),
      0xAC => (1, CPU::xor_a_h),
      0xAD => (1, CPU::xor_a_l),
      0xAE => (2, CPU::xor_a_mhl),
      0xAF => (1, CPU::xor_a_a),
      0xB0 => (1, CPU::or_a_b),
      0xB1 => (1, CPU::or_a_c),
      0xB2 => (1, CPU::or_a_d),
      0xB3 => (1, CPU::or_a_e),
      0xB4 => (1, CPU::or_a_h),
      0xB5 => (1, CPU::or_a_l),
      0xB6 => (2, CPU::or_a_mhl),
      0xB7 => (1, CPU::or_a_a),
      0xB8 => (1, CPU::cp_a_b),
      0xB9 => (1, CPU::cp_a_c),
      0xBA => (1, CPU::cp_a_d),
      0xBB => (1, CPU::cp_a_e),
      0xBC => (1, CPU::cp_a_h),
      0xBD => (1, CPU::cp_a_l),
      0xBE => (2, CPU::cp_a_mhl),
      0xBF => (1, CPU::cp_a_a),
      0xC0 => (5, CPU::ret_nz),
      0xC1 => (3, CPU::pop_bc),
      0xC2 => (4, CPU::jp_nz_nn),
      0xC3 => (4, CPU::jp_nn),
      0xC4 => (6, CPU::call_nz_nn),
      0xC5 => (4, CPU::push_bc),
      0xC6 => (2, CPU::add_a_n),
      0xC7 => (4, CPU::rst_00),
      0xC8 => (5, CPU::ret_z),
      0xC9 => (4, CPU::ret),
      0xCA => (4, CPU::jp_z_nn),
      0xCB => (1, CPU::prefix),
      0xCC => (6, CPU::call_z_nn),
      0xCD => (6, CPU::call_nn),
      0xCE => (2, CPU::adc_a_n),
      0xCF => (4, CPU::rst_08),
      0xD0 => (5, CPU::ret_nc),
      0xD1 => (3, CPU::pop_de),
      0xD2 => (4, CPU::jp_nc_nn),
      0xD3 => (0, CPU::unimplemented),
      0xD4 => (6, CPU::call_nc_nn),
      0xD5 => (4, CPU::push_de),
      0xD6 => (2, CPU::sub_a_n),
      0xD7 => (4, CPU::rst_10),
      0xD8 => (5, CPU::ret_c),
      0xD9 => (4, CPU::reti),
      0xDA => (4, CPU::jp_c_nn),
      0xDB => (1, CPU::unimplemented),
      0xDC => (6, CPU::call_c_nn),
      0xDD => (6, CPU::unimplemented),
      0xDE => (2, CPU::sbc_a_n),
      0xDF => (4, CPU::rst_18),
      0xE0 => (3, CPU::ldh_n_a),
      0xE1 => (3, CPU::pop_hl),
      0xE2 => (2, CPU::ld_mc_a),
      0xE3 => (0, CPU::unimplemented),
      0xE4 => (0, CPU::unimplemented),
      0xE5 => (4, CPU::push_hl),
      0xE6 => (2, CPU::and_a_n),
      0xE7 => (4, CPU::rst_20),
      0xE8 => (4, CPU::add_sp_rn),
      0xE9 => (1, CPU::jp_hl),
      0xEA => (4, CPU::ld_mnn_a),
      0xEB => (0, CPU::unimplemented),
      0xEC => (0, CPU::unimplemented),
      0xED => (0, CPU::unimplemented),
      0xEE => (2, CPU::xor_a_n),
      0xEF => (4, CPU::rst_28),
      0xF0 => (3, CPU::ldh_a_n),
      0xF1 => (3, CPU::pop_af),
      0xF2 => (2, CPU::ld_a_mc),
      0xF3 => (1, CPU::di),
      0xF4 => (0, CPU::unimplemented),
      0xF5 => (4, CPU::push_af),
      0xF6 => (2, CPU::or_a_n),
      0xF7 => (4, CPU::rst_30),
      0xF8 => (3, CPU::ld_hl_sprn),
      0xF9 => (2, CPU::ld_sp_hl),
      0xFA => (4, CPU::ld_a_mnn),
      0xFB => (1, CPU::ei),
      0xFC => (0, CPU::unimplemented),
      0xFD => (0, CPU::unimplemented),
      0xFE => (2, CPU::cp_a_n),
      0xFF => (4, CPU::rst_38),
    }
  }


  /*
  -------------------
    INSTRUCTION SET
  -------------------
  */

  fn unimplemented(&mut self) {
    println!("unimplemented opcode");
  }

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
    self.set_b(res);
  }

  fn dec_b(&mut self) {
    if self.get_b() == 0 {
      self.set_b(0xFF);
      self.set_flag(Flag::Z, false);
      self.set_flag(Flag::N, false);
      self.set_flag(Flag::H, false);
    } else {
      let is_half_carry = (((self.get_b() & 0xf) - (1 & 0xf)) & 0x10) != 0;
      let res = self.get_b() - 1;
      self.set_flag(Flag::Z, res == 0);
      self.set_flag(Flag::N, false);
      self.set_flag(Flag::H, is_half_carry);
      self.set_b(res);
    }
  }

  fn ld_b_n(&mut self) {
    self.set_b(self.read(self.pc));
    self.pc += 1;
    println!("b set to {}", self.get_b());
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
    let total = (self.get_hl() + self.get_bc()) as u32;
    let res = (total & 0xFFFF) as u16;
    let is_half_carry = (((self.get_hl() & 0xFFF) + (self.get_bc() & 0xFFF)) & 0x1000) != 0; 
    self.set_flag(Flag::C, total > 0xFFFF);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_hl(res);
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
    if self.get_c() == 0 {
      self.set_c(0xFF);
      self.set_flag(Flag::Z, false);
      self.set_flag(Flag::N, false);
      self.set_flag(Flag::H, false);
    } else {
      let is_half_carry = (((self.get_c() & 0xf) - (1 & 0xf)) & 0x10) != 0;
      let res = self.get_c() - 1;
      self.set_flag(Flag::Z, res == 0);
      self.set_flag(Flag::N, false);
      self.set_flag(Flag::H, is_half_carry);
      self.set_c(res);
    }
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
    if self.get_d() == 0 {
      self.set_d(0xFF);
      self.set_flag(Flag::Z, false);
      self.set_flag(Flag::N, false);
      self.set_flag(Flag::H, false);
    } else {
      let is_half_carry = (((self.get_d() & 0xf) - (1 & 0xf)) & 0x10) != 0;
      let res = self.get_d() - 1;
      self.set_flag(Flag::Z, res == 0);
      self.set_flag(Flag::N, false);
      self.set_flag(Flag::H, is_half_carry);
      self.set_d(res);
    }
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
    let total = (self.get_hl() + self.get_de()) as u32; 
    let res = (total & 0xFFFF) as u16;
    let is_half_carry = (((self.get_hl() & 0xFFF) + (self.get_de() & 0xFFF)) & 0x1000) != 0; 
    self.set_flag(Flag::C, total > 0xFFFF);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_hl(res);
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
    if !self.get_flag(Flag::Z) {
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
    let total = (self.get_hl() + self.get_hl()) as u32; 
    let res = (total & 0xFFFF) as u16;
    let is_half_carry = (((self.get_hl() & 0xFFF) + (self.get_hl() & 0xFFF)) & 0x1000) != 0; 
    self.set_flag(Flag::C, total > 0xFFFF);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_hl(res);
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

  // HERE
  fn jr_nc_n(&mut self) {
    let signed_int = self.read(self.pc);
    self.pc += 1;
    if self.get_flag(Flag::N) && self.get_flag(Flag::C) {
      if signed_int > 127 {
        self.pc += (signed_int & 0x7F) as u16;
      } else {
        self.pc -= (signed_int & 0x7F) as u16;
      }
    }
  }

  fn ld_sp_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.sp = (hi << 8) + lo;
  }

  fn ldd_mhl_a(&mut self) {
    self.write(self.get_hl(), self.get_a());
    self.set_hl(self.get_hl() - 1);
  }

  fn inc_sp(&mut self) {
    self.sp += 1;
  }

  fn inc_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let is_half_carry = (((byte & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = byte + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let is_half_carry = (((byte & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = byte - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_mhl_n(&mut self) {
    self.write(self.get_hl(), self.read(self.pc));
    self.pc += 1;
  }

  fn scf(&mut self) {
    self.set_flag(Flag::C, true);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
  }

  fn jr_c_n(&mut self) {
    let signed_int = self.read(self.pc);
    self.pc += 1;
    if self.get_flag(Flag::C) {
      if signed_int > 127 {
        self.pc += (signed_int & 0x7F) as u16;
      } else {
        self.pc -= (signed_int & 0x7F) as u16;
      }
    }
  }

  fn add_hl_sp(&mut self) {
    let total = (self.get_hl() + self.sp) as u32; 
    let res = (total & 0xFFFF) as u16;
    let is_half_carry = (((self.get_hl() & 0xFFF) + (self.sp & 0xFFF)) & 0x1000) != 0; 
    self.set_flag(Flag::C, total > 0xFFFF);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_hl(res);
  }

  fn ldd_a_mhl(&mut self) {
    self.set_a(self.read(self.get_hl()));
    self.set_hl(self.get_hl() - 1);
  }

  fn dec_sp(&mut self) {
    self.sp -= 1;
  }

  fn inc_a(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) + (1 & 0xf)) & 0x10) != 0;
    let res = self.get_a() + 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn dec_a(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) - (1 & 0xf)) & 0x10) != 0;
    let res = self.get_a() - 1;
    self.set_flag(Flag::Z, res == 0);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
  }

  fn ld_a_n(&mut self) {
    self.set_a(self.read(self.pc));
    self.pc += 1;
  }

  fn ccf(&mut self) {
    self.set_flag(Flag::C, !self.get_flag(Flag::C));
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::H, true);
  }

  fn ld_b_b(&mut self) {
    self.set_b(self.get_b());
  }

  fn ld_b_c(&mut self) {
    self.set_b(self.get_c());
  }

  fn ld_b_d(&mut self) {
    self.set_b(self.get_d());
  }

  fn ld_b_e(&mut self) {
    self.set_b(self.get_e());
  }

  fn ld_b_h(&mut self) {
    self.set_b(self.get_h());
  }

  fn ld_b_l(&mut self) {
    self.set_b(self.get_l());
  }

  fn ld_b_mhl(&mut self) {
    self.set_b(self.read(self.get_hl()));
  }

  fn ld_b_a(&mut self) {
    self.set_b(self.get_a());
  }

  fn ld_c_b(&mut self) {
    self.set_c(self.get_b());
  }

  fn ld_c_c(&mut self) {
    self.set_c(self.get_c());
  }

  fn ld_c_d(&mut self) {
    self.set_c(self.get_d());
  }

  fn ld_c_e(&mut self) {
    self.set_c(self.get_e());
  }

  fn ld_c_h(&mut self) {
    self.set_c(self.get_h());
  }

  fn ld_c_l(&mut self) {
    self.set_c(self.get_l());
  }

  fn ld_c_mhl(&mut self) {
    self.set_c(self.read(self.get_hl()));
  }

  fn ld_c_a(&mut self) {
    self.set_c(self.get_a());
  }

  fn ld_d_b(&mut self) {
    self.set_d(self.get_b());
  }

  fn ld_d_c(&mut self) {
    self.set_d(self.get_c());
  }

  fn ld_d_d(&mut self) {
    self.set_d(self.get_d());
  }

  fn ld_d_e(&mut self) {
    self.set_d(self.get_e());
  }

  fn ld_d_h(&mut self) {
    self.set_d(self.get_h());
  }

  fn ld_d_l(&mut self) {
    self.set_d(self.get_l());
  }

  fn ld_d_mhl(&mut self) {
    self.set_d(self.read(self.get_hl()));
  }

  fn ld_d_a(&mut self) {
    self.set_b(self.get_a());
  }

  fn ld_e_b(&mut self) {
    self.set_e(self.get_b());
  }

  fn ld_e_c(&mut self) {
    self.set_e(self.get_c());
  }

  fn ld_e_d(&mut self) {
    self.set_e(self.get_d());
  }

  fn ld_e_e(&mut self) {
    self.set_e(self.get_e());
  }

  fn ld_e_h(&mut self) {
    self.set_e(self.get_h());
  }

  fn ld_e_l(&mut self) {
    self.set_e(self.get_l());
  }

  fn ld_e_mhl(&mut self) {
    self.set_e(self.read(self.get_hl()));
  }

  fn ld_e_a(&mut self) {
    self.set_e(self.get_a());
  }

  fn ld_h_b(&mut self) {
    self.set_h(self.get_b());
  }

  fn ld_h_c(&mut self) {
    self.set_h(self.get_c());
  }

  fn ld_h_d(&mut self) {
    self.set_h(self.get_d());
  }

  fn ld_h_e(&mut self) {
    self.set_h(self.get_e());
  }

  fn ld_h_h(&mut self) {
    self.set_h(self.get_h());
  }

  fn ld_h_l(&mut self) {
    self.set_h(self.get_l());
  }

  fn ld_h_mhl(&mut self) {
    self.set_h(self.read(self.get_hl()));
  }

  fn ld_h_a(&mut self) {
    self.set_h(self.get_a());
  }

  fn ld_l_b(&mut self) {
    self.set_l(self.get_b());
  }

  fn ld_l_c(&mut self) {
    self.set_l(self.get_c());
  }

  fn ld_l_d(&mut self) {
    self.set_l(self.get_d());
  }

  fn ld_l_e(&mut self) {
    self.set_l(self.get_e());
  }

  fn ld_l_h(&mut self) {
    self.set_l(self.get_h());
  }

  fn ld_l_l(&mut self) {
    self.set_l(self.get_l());
  }

  fn ld_l_mhl(&mut self) {
    self.set_l(self.read(self.get_hl()));
  }

  fn ld_l_a(&mut self) {
    self.set_l(self.get_a());
  }

  fn ld_mhl_b(&mut self) {
    self.write(self.get_hl(), self.get_b());
  }

  fn ld_mhl_c(&mut self) {
    self.write(self.get_hl(), self.get_c());
  }

  fn ld_mhl_d(&mut self) {
    self.write(self.get_hl(), self.get_d());
  }

  fn ld_mhl_e(&mut self) {
    self.write(self.get_hl(), self.get_e());
  }

  fn ld_mhl_h(&mut self) {
    self.write(self.get_hl(), self.get_h());
  }

  fn ld_mhl_l(&mut self) {
    self.write(self.get_hl(), self.get_l());
  }

  fn halt(&mut self) {
    //TODO: implement halt
  }

  fn ld_mhl_a(&mut self) {
    self.write(self.get_hl(), self.get_a());
  }

  fn ld_a_b(&mut self) {
    self.set_a(self.get_b());
  }

  fn ld_a_c(&mut self) {
    self.set_a(self.get_c());
  }

  fn ld_a_d(&mut self) {
    self.set_a(self.get_d());
  }

  fn ld_a_e(&mut self) {
    self.set_a(self.get_e());
  }

  fn ld_a_h(&mut self) {
    self.set_a(self.get_h());
  }

  fn ld_a_l(&mut self) {
    self.set_a(self.get_l());
  }

  fn ld_a_mhl(&mut self) {
    self.set_a(self.read(self.get_hl()));
  }

  fn ld_a_a(&mut self) {
    self.set_a(self.get_a());
  }
  
  fn add_a_b(&mut self) {
    let total = (self.get_a() + self.get_b()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_b() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn add_a_c(&mut self) {
    let total = (self.get_a() + self.get_c()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_c() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn add_a_d(&mut self) {
    let total = (self.get_a() + self.get_d()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_d() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn add_a_e(&mut self) {
    let total = (self.get_a() + self.get_e()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_e() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn add_a_h(&mut self) {
    let total = (self.get_a() + self.get_h()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_h() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn add_a_l(&mut self) {
    let total = (self.get_a() + self.get_l()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_l() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn add_a_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let total = (self.get_a() + byte) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn add_a_a(&mut self) {
    let total = (self.get_a() + self.get_a()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_a() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn adc_a_b(&mut self) {
    let mut total = (self.get_a() + self.get_b()) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_b() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn adc_a_c(&mut self) {
    let mut total = (self.get_a() + self.get_c()) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_c() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn adc_a_d(&mut self) {
    let mut total = (self.get_a() + self.get_d()) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_d() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn adc_a_e(&mut self) {
    let mut total = (self.get_a() + self.get_e()) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_e() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn adc_a_h(&mut self) {
    let mut total = (self.get_a() + self.get_h()) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_h() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn adc_a_l(&mut self) {
    let mut total = (self.get_a() + self.get_l()) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_l() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn adc_a_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let mut total = (self.get_a() + byte) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn adc_a_a(&mut self) {
    let mut total = (self.get_a() + self.get_a()) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_a() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn sub_a_b(&mut self) {
    let total = (self.get_a() - self.get_b()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_b() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sub_a_c(&mut self) {
    let total = (self.get_a() - self.get_c()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_c() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sub_a_d(&mut self) {
    let total = (self.get_a() - self.get_d()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_d() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sub_a_e(&mut self) {
    let total = (self.get_a() - self.get_e()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_e() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sub_a_h(&mut self) {
    let total = (self.get_a() - self.get_h()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (self.get_h() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sub_a_l(&mut self) {
    let total = (self.get_a() - self.get_l()) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_l() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn sub_a_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let total = (self.get_a() - byte) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sub_a_a(&mut self) {
    self.set_flag(Flag::C, false); 
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, true);
    self.set_a(0);
  }

  fn sbc_a_b(&mut self) {
    let mut total = (self.get_a() - self.get_b()) as u16;
    if self.get_flag(Flag::C) { 
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_b() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sbc_a_c(&mut self) {
    let mut total = (self.get_a() - self.get_c()) as u16;
    if self.get_flag(Flag::C) { 
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_c() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sbc_a_d(&mut self) {
    let mut total = (self.get_a() - self.get_d()) as u16;
    if self.get_flag(Flag::C) { 
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_d() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sbc_a_e(&mut self) {
    let mut total = (self.get_a() - self.get_e()) as u16;
    if self.get_flag(Flag::C) { 
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_e() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sbc_a_h(&mut self) {
    let mut total = (self.get_a() - self.get_h()) as u16;
    if self.get_flag(Flag::C) { 
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_h() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sbc_a_l(&mut self) {
    let mut total = (self.get_a() - self.get_l()) as u16;
    if self.get_flag(Flag::C) {
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_l() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn sbc_a_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let mut total = (self.get_a() - byte) as u16;
    if self.get_flag(Flag::C) {
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }
  
  fn sbc_a_a(&mut self) {
    let mut total = (self.get_a() - self.get_a()) as u16;
    if self.get_flag(Flag::C) {
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_a() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_b(&mut self) {
    let res = self.get_a() & self.get_b();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_c(&mut self) {
    let res = self.get_a() & self.get_c();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_d(&mut self) {
    let res = self.get_a() & self.get_d();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_e(&mut self) {
    let res = self.get_a() & self.get_e();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_h(&mut self) {
    let res = self.get_a() & self.get_h();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_l(&mut self) {
    let res = self.get_a() & self.get_l();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let res = self.get_a() & byte;
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn and_a_a(&mut self) {
    let res = self.get_a() & self.get_a();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_b(&mut self) {
    let res = self.get_a() ^ self.get_b();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_c(&mut self) {
    let res = self.get_a() ^ self.get_c();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_d(&mut self) {
    let res = self.get_a() ^ self.get_d();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_e(&mut self) {
    let res = self.get_a() ^ self.get_e();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_h(&mut self) {
    let res = self.get_a() ^ self.get_h();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_l(&mut self) {
    let res = self.get_a() ^ self.get_l();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_mhl(&mut self) {
    let res = self.get_a() ^ self.read(self.get_hl());
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn xor_a_a(&mut self) {
    let res = self.get_a() ^ self.get_a();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, true);
    self.set_a(res);
  }

  fn or_a_b(&mut self) {
    let res = self.get_a() | self.get_b();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn or_a_c(&mut self) {
    let res = self.get_a() | self.get_c();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn or_a_d(&mut self) {
    let res = self.get_a() | self.get_d();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn or_a_e(&mut self) {
    let res = self.get_a() | self.get_e();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn or_a_h(&mut self) {
    let res = self.get_a() | self.get_h();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn or_a_l(&mut self) {
    let res = self.get_a() | self.get_l();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn or_a_mhl(&mut self) {
    let res = self.get_a() | self.read(self.get_hl());
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn or_a_a(&mut self) {
    let res = self.get_a() | self.get_a();
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn cp_a_b(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_b() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < self.get_b());
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == self.get_b());
  }

  fn cp_a_c(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_c() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < self.get_c());
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == self.get_c());
  }

  fn cp_a_d(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_d() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < self.get_d());
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == self.get_d());
  }

  fn cp_a_e(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_e() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < self.get_e());
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == self.get_e());
  }

  fn cp_a_h(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_h() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < self.get_h());
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == self.get_h());
  }

  fn cp_a_l(&mut self) {
    let is_half_carry = (((self.get_a() & 0xf) - (self.get_l() & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < self.get_l());
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == self.get_l());
  }

  fn cp_a_mhl(&mut self) {
    let byte = self.read(self.get_hl());
    let is_half_carry = (((self.get_a() & 0xf) - (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < byte);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == byte);
  }

  fn cp_a_a(&mut self) {
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, true);
  }
  
  fn ret_nz(&mut self) {
    if !self.get_flag(Flag::Z) {
      let lo = self.read(self.sp) as u16;
      self.sp += 1;
      let hi = self.read(self.sp) as u16;
      self.sp += 1;
      let nn = (hi << 8) + lo;
      self.pc = nn;
    }
  }

  fn pop_bc(&mut self) {
    let lo = self.read(self.sp) as u16;
    self.sp += 1;
    let hi = self.read(self.sp) as u16;
    self.sp += 1;
    let nn = (hi << 8) + lo;
    self.set_bc(nn);
  }

  fn jp_nz_nn(&mut self) {
    let lo = self.read(self.pc);
    self.pc += 1;
    let hi = self.read(self.pc);
    self.pc += 1;
    if !self.get_flag(Flag::Z) {
      self.pc = (hi << 8 + lo) as u16;
    }
  }

  fn jp_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.pc += 1;
    let nn = (hi << 8) + lo;
    self.pc = nn;
  }

  fn call_nz_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.pc += 1;
    let nn = (hi << 8) + lo;
    if !self.get_flag(Flag::Z) {
      self.pc = nn;
      self.sp -= 1;
      self.write(self.sp, hi as u8);
      self.sp -= 1;
      self.write(self.sp, lo as u8);
      self.pc = nn;
    }
  }

  fn push_bc(&mut self) {
    self.sp -= 1;
    self.write(self.sp, self.get_b());
    self.sp -= 1;
    self.write(self.sp, self.get_c());
  }

  fn add_a_n(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let res = (self.get_a() + byte) as u16;
    let is_half_carry = (((self.get_a() & 0xf) + (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, res > 0xFF);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::C, res == 0);
  }

  fn rst_00(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x00 as u16;
  }

  fn ret_z(&mut self) {
    if self.get_flag(Flag::Z) {
      let lo = self.read(self.sp) as u16;
      self.sp += 1;
      let hi = self.read(self.sp) as u16;
      self.sp += 1;
      let nn = (hi << 8) + lo;
      self.pc = nn;
    }
  }

  fn ret(&mut self) {
    let lo = self.read(self.sp) as u16;
    self.sp += 1;
    let hi = self.read(self.sp) as u16;
    self.sp += 1;
    let nn = (hi << 8) + lo;
    self.pc = nn;
  }

  fn jp_z_nn(&mut self) {
    let lo = self.read(self.pc);
    self.pc += 1;
    let hi = self.read(self.pc);
    self.pc += 1;
    if self.get_flag(Flag::Z) {
      self.pc = (hi << 8 + lo) as u16;
    }
  }

  fn prefix(&mut self) {
    println!("prefix");
  }

  fn call_z_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.pc += 1;
    let nn = (hi << 8) + lo;
    if self.get_flag(Flag::Z) {
      self.pc = nn;
      self.sp -= 1;
      self.write(self.sp, hi as u8);
      self.sp -= 1;
      self.write(self.sp, lo as u8);
      self.pc = nn;
    }
  }

  fn call_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    let nn = ((hi << 8) + lo) as u16;
    self.pc += 1;
    self.pc = nn;
    self.sp -= 1;
    self.write(self.sp, hi as u8);
    self.sp -= 1;
    self.write(self.sp, lo as u8);
    self.pc = nn;
  }

  fn adc_a_n(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let mut total = (self.get_a() + byte) as u16;
    if self.get_flag(Flag::C) { total += 1 }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) + (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn rst_08(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x08 as u16;
  }

  fn ret_nc(&mut self) {
    if !self.get_flag(Flag::C) {
      let lo = self.read(self.sp) as u16;
      self.sp += 1;
      let hi = self.read(self.sp) as u16;
      self.sp += 1;
      let nn = (hi << 8) + lo;
      self.pc = nn;
    }
  }

  fn pop_de(&mut self) {
    let lo = self.read(self.sp) as u16;
    self.sp += 1;
    let hi = self.read(self.sp) as u16;
    self.sp += 1;
    let nn = (hi << 8) + lo;
    self.set_de(nn);
  }

  fn jp_nc_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.pc += 1;
    if !self.get_flag(Flag::C) {
      self.pc = (hi << 8) + lo;
    }
  }

  fn call_nc_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.pc += 1;
    let nn = (hi << 8) + lo;
    if !self.get_flag(Flag::C) {
      self.pc = nn;
      self.sp -= 1;
      self.write(self.sp, hi as u8);
      self.sp -= 1;
      self.write(self.sp, lo as u8);
      self.pc = nn;
    }
  }

  fn push_de(&mut self) {
    self.sp -= 1;
    self.write(self.sp, self.get_d());
    self.sp -= 1;
    self.write(self.sp, self.get_e());
  }

  fn sub_a_n(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let total = (self.get_a() - byte) as u16;
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < byte); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn rst_10(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x10 as u16;
  }

  fn ret_c(&mut self) {
    if self.get_flag(Flag::C) {
      let lo = self.read(self.sp) as u16;
      self.sp += 1;
      let hi = self.read(self.sp) as u16;
      self.sp += 1;
      let nn = (hi << 8) + lo;
      self.pc = nn;
    }
  }

  fn reti(&mut self) {
    let lo = self.read(self.sp) as u16;
    self.sp += 1;
    let hi = self.read(self.sp) as u16;
    self.sp += 1;
    let nn = (hi << 8) + lo;
    self.pc = nn;
    self.ime = true;
  }

  fn jp_c_nn(&mut self) {
    let lo = self.read(self.pc);
    self.pc += 1;
    let hi = self.read(self.pc);
    self.pc += 1;
    if self.get_flag(Flag::C) {
      self.pc = (hi << 8 + lo) as u16;
    }
  }

  fn call_c_nn(&mut self) {
    let lo = self.read(self.pc) as u16;
    self.pc += 1;
    let hi = self.read(self.pc) as u16;
    self.pc += 1;
    let nn = (hi << 8) + lo;
    if self.get_flag(Flag::C) {
      self.pc = nn;
      self.sp -= 1;
      self.write(self.sp, hi as u8);
      self.sp -= 1;
      self.write(self.sp, lo as u8);
      self.pc = nn;
    }
  }

  fn sbc_a_n(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let mut total = (self.get_a() - byte) as u16;
    if self.get_flag(Flag::C) {
      if total == 0 {
        total = 0xFF; // Wrap around. 0x00 - 1 --> 0xFF
      } else {
        total -= 1;
      }
    }
    let res = (total & 0xFF) as u8;
    let is_half_carry = (((self.get_a() & 0xf) - (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, total > 0xFF); 
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn rst_18(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x18 as u16;
  }

  fn ldh_n_a(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let addr = 0xFF00 + byte as u16;
    self.write(addr, self.get_a());
  }

  fn pop_hl(&mut self) {
    let lo = self.read(self.sp) as u16;
    self.sp += 1;
    let hi = self.read(self.sp) as u16;
    self.sp += 1;
    let nn = (hi << 8) + lo;
    self.set_hl(nn);
  }

  fn ld_mc_a(&mut self) {
    self.pc += 1;
    let addr = 0xFF00 + self.get_c() as u16;
    self.write(addr, self.get_a());
  }

  fn push_hl(&mut self) {
    self.sp -= 1;
    self.write(self.sp, self.get_h());
    self.sp -= 1;
    self.write(self.sp, self.get_l());
  }

  fn and_a_n(&mut self) {
    let byte = self.read(self.pc);
    let res = self.get_a() & byte;
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, true);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn rst_20(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x20 as u16;
  }

  fn add_sp_rn(&mut self) {
    let signed_int = self.read(self.pc);
    if signed_int > 127 {
      self.sp += (signed_int & 0x7F) as u16;
    } else {
      self.sp -= (signed_int & 0x7F) as u16;
    }
  }

  fn jp_hl(&mut self) {
    self.pc = self.get_hl();
  }

  fn ld_mnn_a(&mut self) {
    let hi = self.read(self.pc);
    self.pc += 1;
    let lo = self.read(self.pc);
    self.pc += 1;
    let addr = (hi << 8 + lo) as u16;
    self.write(addr, self.get_a());
  }

  fn xor_a_n(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let res = self.get_a() ^ byte;
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn rst_28(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x28 as u16;
  }

  fn ldh_a_n(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let addr = 0xFF00 + byte as u16;
    self.set_a(self.read(addr));
  }

  fn pop_af(&mut self) {
    let lo = self.read(self.sp) as u16;
    self.sp += 1;
    let hi = self.read(self.sp) as u16;
    self.sp += 1;
    let nn = (hi << 8) + lo;
    self.set_af(nn);
  }

  fn ld_a_mc(&mut self) {
    self.pc += 1;
    let addr = 0xFF00 + self.get_c() as u16;
    self.set_a(self.read(addr));
  }

  fn di(&mut self) {
    self.ime = false;
  }

  fn push_af(&mut self) {
    self.sp -= 1;
    self.write(self.sp, self.get_a());
    self.sp -= 1;
    self.write(self.sp, self.get_f());
  }

  fn or_a_n(&mut self) {
    let res = self.get_a() | self.read(self.pc);
    self.pc += 1;
    self.set_flag(Flag::C, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::Z, res == 0);
    self.set_a(res);
  }

  fn rst_30(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x30 as u16;
  }

  fn ld_hl_sprn(&mut self) {
    let signed_int = self.read(self.pc);
    if signed_int > 127 {
      self.set_hl(self.sp + (signed_int & 0x7F) as u16);
    } else {
      self.set_hl(self.sp - (signed_int & 0x7F) as u16);
    }
  }

  fn ld_sp_hl(&mut self) {
    self.sp = self.get_hl();
  }

  fn ld_a_mnn(&mut self) {
    let hi = self.read(self.pc);
    self.pc += 1;
    let lo = self.read(self.pc);
    self.pc += 1;
    let addr = (hi << 8 + lo) as u16;
    self.set_a(self.read(addr));
  }

  fn ei(&mut self) {
    self.ime = true;
  }

  fn cp_a_n(&mut self) {
    let byte = self.read(self.pc);
    self.pc += 1;
    let is_half_carry = (((self.get_a() & 0xf) - (byte & 0xf)) & 0x10) != 0;
    self.set_flag(Flag::C, self.get_a() < byte);
    self.set_flag(Flag::H, is_half_carry);
    self.set_flag(Flag::N, true);
    self.set_flag(Flag::Z, self.get_a() == byte);
  }

  fn rst_38(&mut self) {
    self.sp -= 1;
    let hi = (self.pc >> 8) as u8;
    let lo = (self.pc & 0xFF) as u8;
    self.write(self.sp, hi);
    self.sp -= 1;
    self.write(self.sp, lo);
    self.pc = 0x38u16;
  }
}

fn msb(d: u16) -> u8 {
  (d >> 8) as u8
}

fn lsb(d: u16) -> u8 {
  (d & 0xFF) as u8
}
