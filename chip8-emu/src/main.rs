use std::error::Error;

use chip8_emu::cpu::CPU;

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU::new();

    // Initialize registers with test values
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 250;

    let mem = &mut cpu.memory;

    // Program at 0x000: Two CALL instructions to 0x100
    mem[0x000] = 0x21; mem[0x001] = 0x00;  // CALL 0x100
    mem[0x002] = 0x21; mem[0x003] = 0x00;  // CALL 0x100
    mem[0x004] = 0x00; mem[0x005] = 0x00;  // Exit

    // Subroutine at 0x100: ADD operations
    mem[0x100] = 0x80; mem[0x101] = 0x14;  // ADD V0, V1 (V0 = V0 + V1)
    mem[0x102] = 0x80; mem[0x103] = 0x14;  // ADD V0, V1 (V0 = V0 + V1)
    mem[0x104] = 0x00; mem[0x105] = 0xEE;  // RET (return from subroutine)

    // Print initial register state
    dbg!(cpu.registers);

    // Run the program
    cpu.run();

    // Print final register state
    // Expected: V0 = 5 + (10 * 2) + (10 * 2) = 45 (called twice)
    dbg!(&cpu.registers);

    Ok(())
}
