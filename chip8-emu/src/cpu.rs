// CHIP-8 CPU with 16 registers, 4KB memory, and a call stack
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    pub registers: [u8; 16],
    pub memory: [u8; 0x1000],
    position_in_memory: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 0x1000],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    // Main execution loop
    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            // Extract opcode parts
            let vx = ((opcode & 0x0F00) >> 8) as u8; // register x
            let vy = ((opcode & 0x00F0) >> 4) as u8; // register y
            let addr = opcode & 0x0FFF; // 12-bit address
            let kk = (opcode & 0x00FF) as u8; // 8-bit value
            let op_minor = (opcode & 0x000F) as u8; // last nibble

            match opcode {
                0x0000 => return,
                0x00E0 => { /* Clear Screen */ }
                0x00EE => self.ret(),                  // Return from subroutine
                0x1000..=0x1FFF => self.jmp(addr),     // Jump to address
                0x2000..=0x2FFF => self.call(addr),    // Call subroutine
                0x3000..=0x3FFF => self.se(vx, kk),    // Skip if Vx == kk
                0x4000..=0x4FFF => self.sne(vx, kk),   // Skip if Vx != kk
                0x5000..=0x5FFF => self.se_xy(vx, vy), // Skip if Vx == Vy
                0x6000..=0x6FFF => self.ld(vx, kk),    // Vx = kk
                0x7000..=0x7FFF => self.add(vx, kk),   // Vx += kk
                0x8000..=0x8FFF => match op_minor {
                    0x00 => self.ld_xy(vx, vy),  // Vx = Vy
                    0x01 => self.or_xy(vx, vy),  // Vx |= Vy
                    0x02 => self.and_xy(vx, vy), // Vx &= Vy
                    0x03 => self.xor_xy(vx, vy), // Vx ^= Vy
                    0x04 => self.add_xy(vx, vy), // Vx += Vy (with carry)
                    _ => {
                        todo!("opcode: {:04x}", opcode);
                    }
                },
                0x9000..=0x9FFF => self.sne_xy(vx, vy), // Skip if Vx != Vy
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    // Read 16-bit opcode from memory
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }

    // Jump to address
    fn jmp(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    // Call subroutine
    fn call(&mut self, addr: u16) {
        let sp = &mut self.stack_pointer;
        let stack = &mut self.stack;

        if *sp > (*stack).len() {
            panic!("stack overflow")
        }

        (*stack)[*sp] = self.position_in_memory as u16;
        *sp += 1;

        self.position_in_memory = addr as usize;
    }

    // Return from subroutine
    fn ret(&mut self) {
        let sp = &mut self.stack_pointer;

        if *sp == 0 {
            panic!("stack underflow")
        }

        *sp -= 1;
        let call_addr = self.stack[*sp];
        self.position_in_memory = call_addr as usize;
    }

    // Skip if Vx == kk
    fn se(&mut self, vx: u8, kk: u8) {
        if self.registers[vx as usize] == kk {
            self.position_in_memory += 2;
        }
    }

    // Skip if Vx == Vy
    fn se_xy(&mut self, vx: u8, vy: u8) {
        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.position_in_memory += 2;
        }
    }

    // Skip if Vx != kk
    fn sne(&mut self, vx: u8, kk: u8) {
        if self.registers[vx as usize] != kk {
            self.position_in_memory += 2;
        }
    }

    // Skip if Vx != Vy
    fn sne_xy(&mut self, vx: u8, vy: u8) {
        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.position_in_memory += 2;
        }
    }

    // Load immediate: Vx = kk
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    // Load register: Vx = Vy
    fn ld_xy(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] = self.registers[vy as usize];
    }

    // Add immediate: Vx += kk
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    // Add registers: Vx += Vy, VF = carry
    fn add_xy(&mut self, vx: u8, vy: u8) {
        let arg1 = self.registers[vx as usize];
        let arg2 = self.registers[vy as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[vx as usize] = val;

        self.registers[0xF] = if overflow { 1 } else { 0 };
    }

    // Bitwise AND: Vx &= Vy
    fn and_xy(&mut self, vx: u8, vy: u8) {
        let x_ = &self.registers[vx as usize];
        let y_ = &self.registers[vy as usize];

        self.registers[vx as usize] = *x_ & *y_;
    }

    // Bitwise OR: Vx |= Vy
    fn or_xy(&mut self, vx: u8, vy: u8) {
        let x_ = &self.registers[vx as usize];
        let y_ = &self.registers[vy as usize];

        self.registers[vx as usize] = *x_ | *y_;
    }

    // Bitwise XOR: Vx ^= Vy
    fn xor_xy(&mut self, vx: u8, vy: u8) {
        let x_ = &self.registers[vx as usize];
        let y_ = &self.registers[vy as usize];

        self.registers[vx as usize] = *x_ ^ *y_;
    }
}
