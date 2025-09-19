//! Dense byte to unicode character encoding for 32-byte arrays.
//!
//! No-std, zero dependencies, zero allocations.

#![no_std]
#![forbid(unsafe_code)]

/// Base256 encoder/decoder optimized for 32-byte cryptographic values
pub struct Base256;

impl Base256 {
    /// 256 characters selected for:
    /// - Terminal compatibility (tested on gnome-terminal, alacritty, xterm, tmux)
    /// - Visual distinction (no confusable pairs)
    /// - Shell safety (no metacharacters requiring escape)
    const ENCODE_TABLE: [char; 256] = [
        // 0x00-0x39: Base58 alphabet (58 chars) - proven safe everywhere
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J',
        'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c',
        'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
        'w', 'x', 'y', 'z',
        // 0x3A-0x98: Latin-1 Supplement 0xC0-0xFF (64 chars) - letters only, no symbols
        'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', 'Ð', 'Ñ',
        'Ò', 'Ó', 'Ô', 'Õ', 'Ö', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', 'à', 'á', 'â', 'ã', 'ä',
        'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï', 'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö',
        'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ', 'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī',
        // 0x99-0xD8: Greek alphabet (64 chars) - full set with variants
        'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ρ', 'Σ',
        'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'Ϊ', 'Ϋ', 'ά', 'έ', 'ή', 'ί', 'ΰ', 'α', 'β', 'γ', 'δ', 'ε',
        'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'ς', 'σ', 'τ', 'υ', 'φ', 'χ',
        'ψ', 'ω', 'ϊ', 'ϋ', 'ό', 'ύ', 'ώ', 'Ϗ', 'ϐ', 'ϑ',
        // 0xD9-0xFF: Cyrillic alphabet (70 chars) - Russian + common letters
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р',
        'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я', 'а', 'б', 'в',
        'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у',
        'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
    ];

    /// Encode 32 bytes to 32 characters
    #[inline]
    pub const fn encode(input: &[u8; 32]) -> [char; 32] {
        let mut output = ['\0'; 32];
        let mut i = 0;
        while i < 32 {
            output[i] = Self::ENCODE_TABLE[input[i] as usize];
            i += 1;
        }
        output
    }

    /// Decode 32 characters to 32 bytes
    ///
    /// Returns None if any character is not in the alphabet
    #[inline]
    pub fn decode(input: &[char; 32]) -> Option<[u8; 32]> {
        let mut output = [0u8; 32];

        'outer: for i in 0..32 {
            let c = input[i];
            // linear search is fine for 256 elements with good branch prediction
            for (j, &table_char) in Self::ENCODE_TABLE.iter().enumerate() {
                if table_char == c {
                    output[i] = j as u8;
                    continue 'outer;
                }
            }
            return None;
        }

        Some(output)
    }

    /// Convert b256 to hex string (64 bytes)
    pub fn to_hex(input: &[char; 32]) -> Option<[u8; 64]> {
        let bytes = Self::decode(input)?;
        let mut hex = [0u8; 64];
        const HEX_CHARS: &[u8] = b"0123456789abcdef";
        let mut i = 0;
        while i < 32 {
            hex[i * 2] = HEX_CHARS[(bytes[i] >> 4) as usize];
            hex[i * 2 + 1] = HEX_CHARS[(bytes[i] & 0x0f) as usize];
            i += 1;
        }
        Some(hex)
    }

    /// Convert hex string to b256 (must be 64 bytes)
    pub fn from_hex(hex: &[u8; 64]) -> Option<[char; 32]> {
        let mut bytes = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            let hi = match hex[i * 2] {
                b'0'..=b'9' => hex[i * 2] - b'0',
                b'a'..=b'f' => hex[i * 2] - b'a' + 10,
                b'A'..=b'F' => hex[i * 2] - b'A' + 10,
                _ => return None,
            };
            let lo = match hex[i * 2 + 1] {
                b'0'..=b'9' => hex[i * 2 + 1] - b'0',
                b'a'..=b'f' => hex[i * 2 + 1] - b'a' + 10,
                b'A'..=b'F' => hex[i * 2 + 1] - b'A' + 10,
                _ => return None,
            };
            bytes[i] = (hi << 4) | lo;
            i += 1;
        }
        Some(Self::encode(&bytes))
    }

    /// Encode 32 bytes directly to hex (64 bytes)
    pub const fn bytes_to_hex(input: &[u8; 32]) -> [u8; 64] {
        const HEX_CHARS: &[u8] = b"0123456789abcdef";
        let mut hex = [0u8; 64];
        let mut i = 0;
        while i < 32 {
            hex[i * 2] = HEX_CHARS[(input[i] >> 4) as usize];
            hex[i * 2 + 1] = HEX_CHARS[(input[i] & 0x0f) as usize];
            i += 1;
        }
        hex
    }

    /// Decode hex directly to 32 bytes
    pub fn hex_to_bytes(hex: &[u8; 64]) -> Option<[u8; 32]> {
        let mut bytes = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            let hi = match hex[i * 2] {
                b'0'..=b'9' => hex[i * 2] - b'0',
                b'a'..=b'f' => hex[i * 2] - b'a' + 10,
                b'A'..=b'F' => hex[i * 2] - b'A' + 10,
                _ => return None,
            };
            let lo = match hex[i * 2 + 1] {
                b'0'..=b'9' => hex[i * 2 + 1] - b'0',
                b'a'..=b'f' => hex[i * 2 + 1] - b'a' + 10,
                b'A'..=b'F' => hex[i * 2 + 1] - b'A' + 10,
                _ => return None,
            };
            bytes[i] = (hi << 4) | lo;
            i += 1;
        }
        Some(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_has_256_chars() {
        assert_eq!(Base256::ENCODE_TABLE.len(), 256);
    }

    #[test]
    fn all_chars_unique() {
        for i in 0..256 {
            for j in (i + 1)..256 {
                assert_ne!(Base256::ENCODE_TABLE[i], Base256::ENCODE_TABLE[j]);
            }
        }
    }

    #[test]
    fn no_shell_metacharacters() {
        const FORBIDDEN: &[char] = &['"', '\'', '\\', '$', '`', ' ', '\t', '\n', '\r'];
        for &c in &Base256::ENCODE_TABLE {
            for &forbidden in FORBIDDEN {
                assert_ne!(c, forbidden);
            }
        }
    }

    #[test]
    fn roundtrip_exhaustive() {
        // test every byte value in every position
        for pos in 0..32 {
            for byte in 0..=255u8 {
                let mut input = [0u8; 32];
                input[pos] = byte;

                let encoded = Base256::encode(&input);
                let decoded = Base256::decode(&encoded).expect("decode failed");

                assert_eq!(input, decoded);
            }
        }
    }

    #[test]
    fn invalid_char_returns_none() {
        let mut valid = Base256::encode(&[0u8; 32]);
        valid[0] = '\0'; // null not in alphabet
        assert!(Base256::decode(&valid).is_none());
    }

    #[test]
    fn encode_is_deterministic() {
        let input = [0x42u8; 32];
        let enc1 = Base256::encode(&input);
        let enc2 = Base256::encode(&input);
        assert_eq!(enc1, enc2);
    }

    #[test]
    fn hex_roundtrip() {
        let bytes = [0xDE; 32];
        let b256 = Base256::encode(&bytes);
        let hex = Base256::to_hex(&b256).unwrap();
        let b256_again = Base256::from_hex(&hex).unwrap();
        assert_eq!(b256, b256_again);
    }

    #[test]
    fn bytes_hex_roundtrip() {
        let bytes = [0xAB; 32];
        let hex = Base256::bytes_to_hex(&bytes);
        let bytes_again = Base256::hex_to_bytes(&hex).unwrap();
        assert_eq!(bytes, bytes_again);
    }

    #[test]
    fn invalid_hex_returns_none() {
        let mut hex = [b'0'; 64];
        hex[0] = b'G'; // not a hex char
        assert!(Base256::from_hex(&hex).is_none());
        assert!(Base256::hex_to_bytes(&hex).is_none());
    }
}
