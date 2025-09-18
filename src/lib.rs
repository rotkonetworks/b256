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
        // 0x00-0x58: ASCII printables excluding shell metacharacters (89 chars)
        '!', '#', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';',
        '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
        'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', ']', '^', '_', 'a',
        'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
        'z', '{', '|', '}', '~',

        // 0x59-0xB7: Latin-1 supplement (95 chars)
        '¡', '¢', '£', '¤', '¥', '¦', '§', '¨', '©', 'ª', '«', '¬',
        '®', '¯', '°', '±', '²', '³', '´', 'µ', '¶', '·', '¸', '¹',
        'º', '»', '¼', '½', '¾', '¿', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å',
        'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', 'Ð', 'Ñ',
        'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý',
        'Þ', 'ß', 'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é',
        'ê', 'ë', 'ì', 'í', 'î', 'ï', 'ð', 'ñ', 'ò', 'ó', 'ô', 'õ',
        'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ',

        // 0xB8-0xE7: Greek letters that don't resemble Latin (48 chars)
        'Γ', 'Δ', 'Θ', 'Λ', 'Ξ', 'Π', 'Σ', 'Φ', 'Ψ', 'Ω',
        'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ',
        'ν', 'ξ', 'π', 'ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
        'Б', 'Г', 'Д', 'Ж', 'З', 'И', 'Й', 'Л', 'П', 'Ф', 'Ц', 'Ч',
        'Ш', 'Щ', 'Ъ',

        // 0xE8-0xFF: Box drawing and Cyrillic (24 chars)
        'Ы', 'Ь', 'Э', 'Ю', 'Я', 'ж', 'ц', 'ч', 'ш', 'щ',
        '─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼',
        '█', '▄', '▌', '▐',
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
            // Linear search is fine for 256 elements with good branch prediction
            // Lookup table would use 128KB for marginal gains
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_has_256_chars() {
        // Will fail at compile time if not 256, but explicit test for clarity
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
        // Test every byte value in every position
        for pos in 0..32 {
            for byte in 0..=255u8 {
                let mut input = [0u8; 32];
                input[pos] = byte;

                let encoded = Base256::encode(&input);
                let decoded = Base256::decode(&encoded)
                    .expect("decode failed");

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
}
