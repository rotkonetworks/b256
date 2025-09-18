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

let key = [0u8; 32];
let encoded = Base256::encode(&key);  // !#%&()*+,-./0123456789:;<=>?@AB
let decoded = Base256::decode(&encoded).unwrap();
```

CLI:
```bash
# Encode stdin
head -c 32 /dev/urandom | base256

# Decode stdin
echo '_Гî┤gÅωυ3î·ж┤;m<ÆñûЩ;ÏÌЦXXλÎ2M÷·' | base256 -d | xxd

# SSH public key
awk '{print $2}' ~/.ssh/id_ed25519.pub | base64 -d | tail -c32 | base256
```

## Design

- 256 unique unicode characters for 256 byte values
- No expansion: 32 bytes → 32 characters (base64: 32 → 44)
- Shell-safe: no quotes, backslashes, or dollar signs
- No-std, no dependencies

Character set:
- ASCII printables (89) - excluding shell metacharacters
- Latin-1 supplement (95)
- Greek letters (48) - visually distinct from Latin
- Cyrillic/Box drawing (24)

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

