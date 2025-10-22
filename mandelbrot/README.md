# Mandelbrot Set Renderer

A simple ASCII art renderer for the famous Mandelbrot Set fractal, written in Rust.

## What is the Mandelbrot Set?

The Mandelbrot Set is a mathematical set of complex numbers that creates beautiful, infinitely detailed fractal patterns. It's defined by iterating the equation **z = z² + c**, where:
- `z` starts at 0
- `c` is the complex number being tested
- If `z` stays bounded (magnitude ≤ 2) after many iterations, `c` is in the set

## How It Works

The program:
1. Maps each character position to a complex number on the complex plane
2. Tests if that number is in the Mandelbrot set by iterating `z = z² + c`
3. Records how many iterations it takes before the number "escapes" (magnitude > 2)
4. Renders the results as ASCII art, where different characters represent different escape times

## Running the Program

```bash
cargo run
```

## Output

The program renders a 100x24 character ASCII visualization of the Mandelbrot set:

- **Spaces (` `)**: Points that escape immediately (clearly outside the set)
- **Light characters (`.`, `•`, `*`)**: Points that escape relatively quickly
- **Dense characters (`+`, `x`, `$`, `#`, `%`)**: Points that take many iterations to escape or never escape (inside or near the set boundary)

The classic "bulb" shape in the center represents the main body of the Mandelbrot set.

## Parameters

You can modify the rendering in `main()`:

```rust
calculate_mandelbrot(
    1000,   // max_iters: Maximum iterations before giving up
    -2.0,   // x_min: Left edge of the complex plane
    1.0,    // x_max: Right edge
    -1.0,   // y_min: Bottom edge
    1.0,    // y_max: Top edge
    100,    // width: Characters wide
    24      // height: Lines tall
)
```

## Dependencies

- **num**: Provides complex number support

## Learn More

- [Mandelbrot Set on Wikipedia](https://en.wikipedia.org/wiki/Mandelbrot_set)
- The most interesting details are at the boundary of the set - try zooming in by adjusting the coordinate ranges!
