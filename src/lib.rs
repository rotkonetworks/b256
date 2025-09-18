//! Dense byte to unicode character encoding
//!
//! Maps 256 bytes to carefully selected unicode blocks that:
//! - survive copy/paste across all major platforms
//! - are visually distinct
//! - use single-width characters
//! - available in common monospace fonts

#![no_std]

/// Base256 encoder/decoder for 32-byte arrays
pub struct Base256;

impl Base256 {
    const ALPHABET: [char; 256] = {
        let mut chars = ['\0'; 256];
        let mut idx = 0;

        // ascii printables (95 chars)
        let mut c = 0x20u8;
        while c <= 0x7E {
            chars[idx] = c as char;
            idx += 1;
            c += 1;
        }

        // latin-1 supplement (95 chars, skip soft hyphen)
        c = 0xA0;
        loop {
            if c != 0xAD {
                chars[idx] = c as char;
                idx += 1;
            }
            if c == 0xFF {
                break;
            }
            c += 1;
        }

        // greek lowercase (25 chars)
        let mut i = 0x03B1u32;
        while i <= 0x03C9 {
            chars[idx] = unsafe { char::from_u32_unchecked(i) };
            idx += 1;
            i += 1;
        }

        // cyrillic subset (25 chars)
        let cyrillic = [
            0x0410, 0x0411, 0x0412, 0x0413, 0x0414, 0x0415, 0x0416, 0x0417, 0x0418, 0x0419, 0x041A,
            0x041B, 0x041C, 0x041D, 0x041E, 0x041F, 0x0420, 0x0421, 0x0422, 0x0423, 0x0424, 0x0425,
            0x0426, 0x0427, 0x0436,
        ];
        let mut i = 0;
        while i < 25 {
            chars[idx] = unsafe { char::from_u32_unchecked(cyrillic[i]) };
            idx += 1;
            i += 1;
        }

        // math symbols (16 chars)
        let math = [
            0x2200, 0x2202, 0x2203, 0x2205, 0x2206, 0x2207, 0x2208, 0x220B, 0x2211, 0x2212, 0x2217,
            0x221A, 0x221E, 0x2229, 0x222A, 0x2264,
        ];
        i = 0;
        while i < 16 {
            chars[idx] = unsafe { char::from_u32_unchecked(math[i]) };
            idx += 1;
            i += 1;
        }

        chars
    };

    /// Encode 32 bytes to 32 unicode characters
    ///
    /// # Example
    /// ```
    /// let key = [0u8; 32];
    /// let encoded = b256::Base256::encode(&key);
    /// assert_eq!(encoded.len(), 32);
    /// ```
    pub fn encode(input: &[u8; 32]) -> [char; 32] {
        let mut out = ['\0'; 32];
        let mut i = 0;
        while i < 32 {
            out[i] = Self::ALPHABET[input[i] as usize];
            i += 1;
        }
        out
    }

    /// Decode 32 unicode characters to 32 bytes
    ///
    /// Returns None if any character is not in the alphabet.
    ///
    /// # Example
    /// ```
    /// let encoded = [' '; 32];  // 32 spaces (0x20)
    /// let decoded = b256::Base256::decode(&encoded).unwrap();
    /// assert_eq!(decoded, [0u8; 32]);
    /// ```
    pub fn decode(input: &[char; 32]) -> Option<[u8; 32]> {
        let mut result = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            let c = input[i];
            let mut j = 0;
            let mut found = false;
            while j < 256 {
                if Self::ALPHABET[j] == c {
                    result[i] = j as u8;
                    found = true;
                    break;
                }
                j += 1;
            }
            if !found {
                return None;
            }
            i += 1;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let key = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x0f, 0x1e, 0x2d, 0x3c,
            0x4b, 0x5a, 0x69, 0x78,
        ];
        let encoded = Base256::encode(&key);
        let decoded = Base256::decode(&encoded).unwrap();
        assert_eq!(key, decoded);
    }

    #[test]
    fn decode_invalid() {
        let mut invalid = [' '; 32];
        invalid[0] = '\x00';
        assert!(Base256::decode(&invalid).is_none());
    }
}
