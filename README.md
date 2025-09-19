# base256

Dense byte-to-unicode encoding. 32 bytes → 32 characters.

```rust
// Before: Debug output for [u8; 32]
[58, 216, 165, 65, 124, 141, 110, 247, 71, 125, 151, 247, 52, 203, 133, 41,
 110, 37, 2, 225, 199, 129, 236, 58, 239, 142, 11, 154, 90, 205, 222, 11]

// After: base256 encoding
_Гî┤gÅωυ3î·ж┤;m<ÆñûЩ;ÏÌЦXXλÎ2M÷·
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
let encoded = Base256::encode(&key);  // !#%&()*+,-./0123456789:;<=>?@AB
let decoded = Base256::decode(&encoded).unwrap();

// Hex conversions (new in 0.3.0)
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
echo '_Гî┤gÅωυ3î·ж┤;m<ÆñûЩ;ÏÌЦXXλÎ2M÷·' | base256 -d | xxd

# Encode to hex (new in 0.3.0)
head -c 32 /dev/urandom | base256 -x

# Convert hex to base256
echo '3ad8a5417c8d6ef7477d97f734cb85296e2502e1c781ec3aef8e0b9a5acdde0b' | base256 -X

# Convert base256 to hex
echo '_Гî┤gÅωυ3î·ж┤;m<ÆñûЩ;ÏÌЦXXλÎ2M÷·' | base256 -dx

# Convert hex to raw bytes
echo '3ad8a5417c8d6ef7477d97f734cb85296e2502e1c781ec3aef8e0b9a5acdde0b' | base256 -Xd

# SSH public key to base256
awk '{print $2}' ~/.ssh/id_ed25519.pub | base64 -d | tail -c32 | base256

# SSH public key to hex
awk '{print $2}' ~/.ssh/id_ed25519.pub | base64 -d | tail -c32 | base256 -x
```

## What's New in 0.3.0

- **Hex support**: Convert between base256 and hex formats
- **Flexible CLI**: Combined flags like `-dx` for base256→hex conversion
- **Direct conversions**: `bytes_to_hex()` and `hex_to_bytes()` skip base256 entirely
- **Universal interop**: Use hex when interfacing with tools that don't support base256

## Design

- 256 unique unicode characters for 256 byte values
- No expansion: 32 bytes → 32 characters (base64: 32 → 44, hex: 32 → 64)
- Shell-safe: no quotes, backslashes, or dollar signs
- No-std, no dependencies, no allocations

Character set:
- ASCII printables (89) - excluding shell metacharacters
- Latin-1 supplement (95)
- Greek letters (48) - visually distinct from Latin
- Cyrillic/Box drawing (24)

## When to use

**Use base256 for:**
- Ed25519/Curve25519 public keys as human-visible identifiers
- SHA-256 hashes in logs or terminal output
- P2P network node identifiers
- Any 32-byte value where visual length matters

**Use hex for:**
- Interoperability with existing tools
- Copy-paste in environments with poor Unicode support
- Debugging and development

**Don't use for:**
- Variable-length data
- Binary formats requiring specific encoding standards
- Non-unicode environments

## Examples

```bash
# Generate a keypair identifier
openssl genpkey -algorithm ed25519 | \
  openssl pkey -pubout -outform DER | \
  tail -c32 | \
  base256

# Compare encodings of the same key
KEY=$(head -c32 /dev/urandom)
echo "$KEY" | base256      # 32 chars: ΓΨ₤ωυ...
echo "$KEY" | base256 -x    # 64 chars: 3fa8b2...
echo "$KEY" | base64        # 44 chars: P6iy1p...==

# Quick conversion between formats
echo "existing_hex_key_3fa8..." | base256 -X  # hex to base256
echo "ΓΨ₤ωυ..." | base256 -dx                 # base256 to hex
```

## License

MIT OR Apache-2.0
