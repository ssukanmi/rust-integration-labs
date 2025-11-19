# progress-iter

A Rust library demonstrating **type-driven API design** and the **type-state pattern** through progress bar iterators.

## Overview

`progress-iter` provides iterator extensions that display progress indicators as you iterate over collections. The library uses compile-time type-safety to ensure you can only call valid methods based on the iterator's state.

## Type-State Pattern

This library demonstrates the **type-state pattern**, where different states of an object are encoded as different types. This allows the compiler to enforce valid state transitions at compile time rather than runtime.

### States

The `Progress` iterator has two type states:

- **`Progress<Iter, Unbounded>`** - For iterators without a known length
- **`Progress<Iter, Bounded>`** - For iterators with a known length

```rust
pub struct Progress<Iter, Bound = Unbounded> {
    iter: Iter,
    i: usize,
    bound: Bound,
}
```

### Type-Driven API Design

The API design prevents invalid operations at compile time:

1. **Unbounded state** - Only `ExactSizeIterator` types can transition to bounded:
   ```rust
   impl<Iter> Progress<Iter, Unbounded>
   where
       Iter: ExactSizeIterator,
   {
       pub fn with_bound(self) -> Progress<Iter, Bounded> {
           // Transition from Unbounded to Bounded
       }
   }
   ```

2. **Bounded state** - Only bounded progress bars can customize delimiters:
   ```rust
   impl<Iter> Progress<Iter, Bounded> {
       pub fn with_delims(mut self, delims: (char, char)) -> Self {
           // Only available in Bounded state
       }
   }
   ```

This design ensures:
- You can't set delimiters on unbounded iterators (compile error)
- You can't create a bounded progress bar from an iterator without a known size
- The compiler guides you to use the API correctly

## Usage

### Basic unbounded progress (infinite iterator)

```rust
use progress_iter::ProgressIteratorExt;

for _ in (0..).progress() {
    // Display: ****** (grows infinitely)
}
```

### Bounded progress with known size

```rust
use progress_iter::ProgressIteratorExt;
use std::{thread::sleep, time::Duration};

let items = [1, 2, 3, 4, 5];

for item in items.iter().progress().with_bound() {
    // Display: [***  ]
    sleep(Duration::from_millis(100));
}
```

### Custom delimiters (only available for bounded)

```rust
use progress_iter::ProgressIteratorExt;

let items = [1, 2, 3];

// This works - bounded state allows with_delims
for item in items.iter().progress().with_bound().with_delims(('|', '|')) {
    // Display: |***|
}

// This would be a compile error:
// for item in (0..).progress().with_delims(('|', '|')) {
//     ^^^^^^^^^^ no method named `with_delims` in unbounded state
// }
```

## Running Examples

```bash
cargo run --example demo -p progress-iter
```

## Benefits of Type-State Pattern

1. **Compile-time safety** - Invalid state transitions are caught by the compiler
2. **Zero runtime cost** - No runtime checks needed for state validity
3. **Self-documenting API** - The type system guides users to correct usage
4. **Impossible states are unrepresentable** - Can't have delimiters on unbounded progress

## Learning Resources

This library demonstrates:
- Type-state pattern in Rust
- Zero-cost abstractions
- Trait-based API design
- Generic type parameters for state encoding
- Builder pattern with type safety
