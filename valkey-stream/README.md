# Valkey Stream Examples

Rust examples for working with Valkey/Redis Streams using the `redis` crate.

## Setup

Start Valkey server:
```bash
docker run --rm -p 6379:6379 valkey/valkey:latest
```

## Running Examples

```bash
# Run an example
cargo run --example [example-name]

# Examples: vk-simple, vk-xadd, vk-async
```

**Available examples:**
- `vk-simple` - Basic SET/GET operations
- `vk-xadd` - Stream operations: XADD, XREAD, XTRIM, DEL
- `vk-async` - Async producer-consumer with concurrent tasks

## Resources

- [Valkey Streams Intro](https://valkey.io/topics/streams-intro/)
- [redis-rs](https://docs.rs/redis/)
