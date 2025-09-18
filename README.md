# base256

Dense byte-to-unicode encoding. Maps 256 bytes to 256 distinct unicode characters.

## What is base256

A compact encoding scheme for 32-byte values (keys, hashes) that:
- Produces exactly 32 unicode characters from 32 bytes
- Uses visually distinct characters
- Survives copy/paste across platforms
- Works in terminals and monospace fonts

## What base256 is not

- Not base64 (which expands data by 33%)
- Not for arbitrary-length data (fixed 32-byte arrays only)
- Not for cryptographic security (just encoding)
- Not latex-compatible (uses unicode blocks)

## Installation

```bash
cargo add b256
```

## Library usage

```rust
use b256::Base256;

let key = [0u8; 32];
let encoded = Base256::encode(&key);  // [char; 32]
let decoded = Base256::decode(&encoded).unwrap();
assert_eq!(key, decoded);
```

## Command-line usage

Install the CLI:
```bash
cargo install b256
```

Use base256:
```bash
# encode 32 bytes from stdin
head -c 32 /dev/urandom | base256

# decode back to binary
echo "encoded_string" | base256 -d | xxd

# round-trip
head -c 32 /dev/urandom | base256 | base256 -d | xxd

# help
base256 --help
```

## Character set

- ASCII printables (95)
- Latin-1 supplement (95)
- Greek lowercase (25)
- Cyrillic subset (25)
- Math symbols (16)

Total: 256 unique characters optimized for visual distinction and font support.

## Use cases

### When to use base256
- Displaying cryptographic keys in terminals
- Compact representation of 32-byte identifiers
- Copy-pasteable binary data that needs to stay compact
- Ed25519 public keys, SHA-256 hashes

### When not to use base256
- Variable-length data
- Systems requiring base64 compatibility
- Documents requiring ASCII-only encoding
- Data that might be processed by legacy systems

## License

MIT OR Apache-2.0
