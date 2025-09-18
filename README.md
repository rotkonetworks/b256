# base256

Dense byte-to-unicode encoding. 32 bytes → 32 characters.

```rust
// Before: Debug output for [u8; 32]
[171, 205, 239, 18, 34, 56, 78, 90, 123, 145, 167, 189, 201, 223, 234, 245, 
 12, 34, 56, 78, 90, 112, 134, 156, 178, 190, 212, 234, 245, 250, 251, 252]

// After: base256 encoding  
«Íï"8LZy£½Éßêõ\x0c"8LZpÄ³Ôêõúûü
```

## Install

Library:
```bash
cargo add b256
```

CLI:
```bash
cargo install b256
```

## Use

Library:
```rust
use b256::Base256;

let key = [0u8; 32];
let encoded = Base256::encode(&key);
let decoded = Base256::decode(&encoded).unwrap();
```

CLI:
```bash
# Encode stdin
head -c 32 /dev/urandom | base256

# Decode stdin  
echo "«Íï"8LZy£½Éßêõ\x0c"8LZpÄ³Ôêõúûü" | base256 -d | xxd

# SSH public key
awk '{print $2}' ~/.ssh/id_ed25519.pub | base64 -d | tail -c32 | base256
```

## Design

- 256 unique unicode characters for 256 byte values
- No expansion: 32 bytes → 32 characters (base64: 32 → 44)
- Characters selected for terminal compatibility and visual distinction
- No-std, no dependencies

Character set:
- ASCII printables (95)
- Latin-1 supplement (95)  
- Greek lowercase (25)
- Cyrillic subset (25)
- Math symbols (16)

## When to use

Use for:
- Ed25519/Curve25519 keys
- SHA-256 hashes
- 32-byte identifiers in logs
- Any fixed 32-byte value needing compact display

Don't use for:
- Variable-length data
- Systems requiring base64
- Non-unicode environments

## License

MIT OR Apache-2.0
