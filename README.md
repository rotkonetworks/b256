# base256

Dense byte-to-unicode encoding. 32 bytes → 32 characters.

```rust
// Before: Debug output for [u8; 32]
[58, 216, 165, 65, 124, 141, 110, 247, 71, 125, 151, 247, 52, 203, 133, 41,
 110, 37, 2, 225, 199, 129, 236, 58, 239, 142, 11, 154, 90, 205, 222, 11]

// After: base256 encoding
ЩóΓÎLÄμöÊLχ9ρÞ4ωÒmiЁBøάЩηÄθÓψÏÇΔa
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

// Basic encoding/decoding
let key = [0u8; 32];
let encoded = Base256::encode(&key);  // 111111111111111111111111111111111
let decoded = Base256::decode(&encoded).unwrap();

// String parsing (new in 0.4.0)
let valid = Base256::is_valid("ЩóΓÎLÄμöÊLχ9ρÞ4ωÒmiЁBøάЩηÄθÓψÏÇΔa");
let bytes = Base256::parse("ЩóΓÎLÄμöÊLχ9ρÞ4ωÒmiЁBøάЩηÄθÓψÏÇΔa").unwrap();

// Hex conversions
let hex_bytes = Base256::to_hex(&encoded).unwrap();  // [48, 48, 48, ...] (64 bytes)
let from_hex = Base256::from_hex(&hex_bytes).unwrap();  // back to base256

// Direct byte-hex conversions
let hex = Base256::bytes_to_hex(&key);  // "0000...0000" as bytes
let bytes = Base256::hex_to_bytes(&hex).unwrap();
```

CLI:
```bash
# Encode stdin to base256 (default)
head -c 32 /dev/urandom | base256

# Decode base256 to raw bytes
echo 'ЩóΓÎLÄμöÊLχ9ρÞ4ωÒmiЁBøάЩηÄθÓψÏÇΔa' | base256 -d | xxd

# Encode to hex
head -c 32 /dev/urandom | base256 -x

# Convert hex to base256
echo '3ad8a5417c8d6ef7477d97f734cb85296e2502e1c781ec3aef8e0b9a5acdde0b' | base256 -X

# Convert base256 to hex
echo 'ЩóΓÎLÄμöÊLχ9ρÞ4ωÒmiЁBøάЩηÄθÓψÏÇΔa' | base256 -dx

# SSH public key to base256
awk '{print $2}' ~/.ssh/id_ed25519.pub | base64 -d | tail -c32 | base256
```

## Design

- **true 1:1 mapping**: 32 bytes → 32 characters (always)
- **shell-safe**: Base58 core avoids ALL shell metacharacters
- **no-std**: Zero dependencies, zero allocations
- **type-safe**: Use `[char; 32]` arrays, not variable-length strings

Character set (256 total):
- Base58 alphabet (58) - proven shell-safe
- Latin-1 letters (68) - no symbols
- Greek alphabet (64) - full set
- Cyrillic letters (66) - Russian alphabet

## When to use

**Use base256 for:**
- Ed25519/Curve25519 public keys
- SHA-256 hashes in terminal output
- Any 32-byte value displayed to humans

**Use hex for:**
- Maximum compatibility
- Machine-to-machine protocols
- Debugging binary data

## Examples

```bash
# Generate ed25519 public key identifier
openssl genpkey -algorithm ed25519 | \
  openssl pkey -pubout -outform DER | \
  tail -c32 | \
  base256

# Output: sοΤQÞвύώБρÄUÁπΨQΞeRϊyϏjϋнХÀēāÄnт

# Compare all encodings
KEY=$(head -c32 /dev/urandom)
echo "$KEY" | base256    # 32 chars: ΓΨΒωυ...
echo "$KEY" | base64     # 44 chars: P6iy1p...==
echo "$KEY" | xxd -p     # 64 chars: 3ad8a5...
```

## License

MIT OR Apache-2.0
