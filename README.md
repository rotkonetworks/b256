# b256

Dense byte-to-unicode encoding. Maps 256 bytes to 256 distinct unicode characters.

## What this is

A compact encoding scheme for 32-byte values (keys, hashes) that:
- Produces exactly 32 unicode characters from 32 bytes
- Uses visually distinct characters
- Survives copy/paste across platforms
- Works in terminals and monospace fonts

## What this is not

- Not base64 (that expands data by 33%)
- Not for arbitrary-length data (fixed 32-byte arrays only)
- Not for cryptographic security (just encoding)
- Not latex-compatible (uses unicode blocks)

## Library usage

```rust
use b256::Base256;

let key = [0u8; 32];
let encoded = Base256::encode(&key);  // [char; 32]
let decoded = Base256::decode(&encoded).unwrap();
assert_eq!(key, decoded);
```

## Binary usage

```bash
# encode 32 bytes from stdin
head -c 32 /dev/urandom | b256

# decode back to binary
echo "encoded_string" | b256 | xxd

# round-trip
head -c 32 /dev/urandom | b256 | b256 | xxd
```

## Character set

- ASCII printables (95)
- Latin-1 supplement (95)  
- Greek lowercase (25)
- Cyrillic subset (25)
- Math symbols (16)

Total: 256 characters, no duplicates, no ambiguous glyphs.

## When to use

- Displaying cryptographic keys in terminals
- Compact representation of 32-byte identifiers
- Copy-pasteable binary data

## When not to use

- Variable-length data
- Production systems requiring base64 compatibility
- Documents requiring ASCII-only
