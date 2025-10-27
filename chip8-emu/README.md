# CHIP-8 Emulator

A simple CHIP-8 virtual machine emulator written in Rust.

## What is CHIP-8?

CHIP-8 is an interpreted programming language developed in the 1970s for programming games on early microcomputers. It features:

- 16 8-bit registers (V0-VF)
- 4KB of RAM
- 16-level call stack
- Simple instruction set for graphics and games

## Features

This emulator currently implements:

- Core CPU with register and memory management
- Opcode decoding and execution
- Subroutine calls and returns (CALL/RET)
- Conditional branching (SE/SNE for immediate and register values)
- Arithmetic operations (ADD with and without carry)
- Bitwise operations (AND, OR, XOR)
- Register load/store operations

## Usage

```rust
use chip8_emu::cpu::CPU;

let mut cpu = CPU::new();

// Load a program into memory
cpu.memory[0x200] = 0x61; // LD V1, 0x42
cpu.memory[0x201] = 0x42;

// Run the program
cpu.run();
```

## Implemented Opcodes

| Opcode | Instruction | Description |
|--------|-------------|-------------|
| 00EE | RET | Return from subroutine |
| 1nnn | JP addr | Jump to address |
| 2nnn | CALL addr | Call subroutine |
| 3xkk | SE Vx, byte | Skip if Vx == kk |
| 4xkk | SNE Vx, byte | Skip if Vx != kk |
| 5xy0 | SE Vx, Vy | Skip if Vx == Vy |
| 6xkk | LD Vx, byte | Set Vx = kk |
| 7xkk | ADD Vx, byte | Add kk to Vx |
| 8xy0 | LD Vx, Vy | Set Vx = Vy |
| 8xy1 | OR Vx, Vy | Vx = Vx OR Vy |
| 8xy2 | AND Vx, Vy | Vx = Vx AND Vy |
| 8xy3 | XOR Vx, Vy | Vx = Vx XOR Vy |
| 8xy4 | ADD Vx, Vy | Vx = Vx + Vy (with carry flag) |
| 9xy0 | SNE Vx, Vy | Skip if Vx != Vy |

## Running the Example

```bash
cargo run -p chip8-emu
```

## References

- [CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Wikipedia: CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
