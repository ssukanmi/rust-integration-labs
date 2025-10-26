# Bit Patterns

A Rust library exploring binary representations and bit-level operations for different numeric types.

## Features

### 🔢 IEEE 754 Float Analysis
Decompose and reconstruct 32-bit floating-point numbers (`f32`) to understand their internal structure:
- Extract sign, exponent, and mantissa bits
- Decode raw bits into actual values
- Reconstruct floats from components

### 🎯 Q7 Fixed-Point Numbers
A simple fixed-point number type with 7 fractional bits, representing values in the range `[-1.0, 1.0]`.

### 🎲 Pseudo-Random Generation
Demonstrates bit manipulation techniques for generating pseudo-random numbers from a byte seed.

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
bit-patterns = { path = "../bit-patterns" }
```

### Examples

#### IEEE 754 Float Decomposition
```rust
use bit_patterns::float;

let n: f32 = 42.42;
let (sign, exp, frac) = float::to_parts(n);
let (sign_val, exp_val, mant_val) = float::decode(sign, exp, frac);
let reconstructed = float::from_parts(sign_val, exp_val, mant_val);

println!("Original: {}", n);
println!("Reconstructed: {}", reconstructed);
```

#### Q7 Fixed-Point
```rust
use bit_patterns::q7::Q7;

let fixed = Q7::from(0.7_f32);
let float = f32::from(fixed);
println!("Q7({:?}) = {}", fixed, float);
```

#### Pseudo-Random Numbers
```rust
use bit_patterns::random;

let rand_val = random::mock_rand(0x7F);
println!("Random: {}", rand_val);
```

## Running the Demo

```bash
cargo run
```

## Module Structure

- `float` - IEEE 754 floating-point operations
- `q7` - Q7 fixed-point number type
- `random` - Pseudo-random number generation

## Learning Resources

This project demonstrates:
- Bitwise operations (`>>`, `<<`, `&`, `|`)
- IEEE 754 floating-point representation
- Fixed-point arithmetic
- Bit manipulation techniques
- Rust trait implementations (`From`, `Into`)

## License

MIT
