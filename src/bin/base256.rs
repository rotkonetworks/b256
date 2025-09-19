// src/bin/base256.rs
use std::env;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("base256 - dense byte to unicode encoding");
        println!();
        println!("Usage: base256 [OPTIONS]");
        println!();
        println!("Options:");
        println!("  -e, --encode       Encode raw bytes to base256 (default)");
        println!("  -d, --decode       Decode base256 to raw bytes");
        println!("  -x, --hex          Use hex format for output");
        println!("  -X, --from-hex     Read hex format as input");
        println!("  -h, --help         Show this help message");
        println!();
        println!("Examples:");
        println!("  # Encode 32 bytes to base256");
        println!("  head -c 32 /dev/urandom | base256");
        println!();
        println!("  # Decode base256 to raw bytes");
        println!("  echo '_Гî┤gÅωυ3î·ж┤;m<ÆñûЩ;ÏÌЦXXλÎ2M÷·' | base256 -d | xxd");
        println!();
        println!("  # Encode bytes to hex");
        println!("  head -c 32 /dev/urandom | base256 -x");
        println!();
        println!("  # Convert hex to base256");
        println!(
            "  echo '3ad8a5417c8d6ef7477d97f734cb85296e2502e1c781ec3aef8e0b9a5acdde0b' | base256 -X"
        );
        println!();
        println!("  # Convert base256 to hex");
        println!("  echo '_Гî┤gÅωυ3î·ж┤;m<ÆñûЩ;ÏÌЦXXλÎ2M÷·' | base256 -dx");
        println!();
        println!("  # Convert hex to raw bytes");
        println!(
            "  echo '3ad8a5417c8d6ef7477d97f734cb85296e2502e1c781ec3aef8e0b9a5acdde0b' | base256 -Xd"
        );
        return;
    }

    // parse flags properly
    let mut decode = false;
    let mut hex_out = false;
    let mut hex_in = false;

    for arg in &args[1..] {
        if arg.starts_with('-') && !arg.starts_with("--") {
            for ch in arg.chars().skip(1) {
                match ch {
                    'd' => decode = true,
                    'x' => hex_out = true,
                    'X' => hex_in = true,
                    'e' => {} // encode is default
                    _ => {}
                }
            }
        } else if arg == "--decode" {
            decode = true;
        } else if arg == "--hex" {
            hex_out = true;
        } else if arg == "--from-hex" {
            hex_in = true;
        } else if arg == "--encode" {
            // default
        }
    }

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // handle empty input
    if input.is_empty() {
        return;
    }

    // trim newline if present
    let trimmed = if input.ends_with(b"\n") {
        &input[..input.len() - 1]
    } else {
        &input[..]
    };

    match (hex_in, decode, hex_out) {
        // hex -> b256
        (true, false, false) => {
            if trimmed.len() != 64 {
                eprintln!("error: expected 64 hex chars, got {}", trimmed.len());
                std::process::exit(1);
            }
            let mut hex_array = [0u8; 64];
            hex_array.copy_from_slice(&trimmed[..64]);
            match b256::Base256::from_hex(&hex_array) {
                Some(encoded) => {
                    for c in &encoded {
                        print!("{}", c);
                    }
                    println!();
                }
                None => {
                    eprintln!("error: invalid hex input");
                    std::process::exit(1);
                }
            }
        }
        // hex -> raw
        (true, true, false) => {
            if trimmed.len() != 64 {
                eprintln!("error: expected 64 hex chars, got {}", trimmed.len());
                std::process::exit(1);
            }
            let mut hex_array = [0u8; 64];
            hex_array.copy_from_slice(&trimmed[..64]);
            match b256::Base256::hex_to_bytes(&hex_array) {
                Some(bytes) => {
                    io::stdout().write_all(&bytes).unwrap();
                }
                None => {
                    eprintln!("error: invalid hex input");
                    std::process::exit(1);
                }
            }
        }
        // hex -> hex (validate and passthrough)
        (true, false, true) => {
            if trimmed.len() != 64 {
                eprintln!("error: expected 64 hex chars, got {}", trimmed.len());
                std::process::exit(1);
            }
            let mut hex_array = [0u8; 64];
            hex_array.copy_from_slice(&trimmed[..64]);
            if b256::Base256::hex_to_bytes(&hex_array).is_none() {
                eprintln!("error: invalid hex input");
                std::process::exit(1);
            }
            io::stdout().write_all(trimmed).unwrap();
            println!();
        }
        // b256 -> hex
        (false, true, true) => {
            let s = std::str::from_utf8(trimmed).unwrap_or_else(|_| {
                eprintln!("error: invalid UTF-8 input");
                std::process::exit(1);
            });
            let chars: Vec<char> = s.chars().collect();
            if chars.len() != 32 {
                eprintln!("error: expected 32 chars, got {}", chars.len());
                std::process::exit(1);
            }
            let mut char_array = ['\0'; 32];
            char_array.copy_from_slice(&chars);
            match b256::Base256::to_hex(&char_array) {
                Some(hex) => {
                    io::stdout().write_all(&hex).unwrap();
                    println!();
                }
                None => {
                    eprintln!("error: invalid base256 character");
                    std::process::exit(1);
                }
            }
        }
        // b256 -> raw
        (false, true, false) => {
            let s = std::str::from_utf8(trimmed).unwrap_or_else(|_| {
                eprintln!("error: invalid UTF-8 input");
                std::process::exit(1);
            });
            let chars: Vec<char> = s.chars().collect();
            if chars.len() != 32 {
                eprintln!("error: expected 32 chars, got {}", chars.len());
                std::process::exit(1);
            }
            let mut char_array = ['\0'; 32];
            char_array.copy_from_slice(&chars);
            match b256::Base256::decode(&char_array) {
                Some(bytes) => {
                    io::stdout().write_all(&bytes).unwrap();
                }
                None => {
                    eprintln!("error: invalid base256 character");
                    std::process::exit(1);
                }
            }
        }
        // raw -> hex
        (false, false, true) => {
            if trimmed.len() != 32 {
                eprintln!("error: expected 32 bytes, got {}", trimmed.len());
                std::process::exit(1);
            }
            let mut byte_array = [0u8; 32];
            byte_array.copy_from_slice(&trimmed[..32]);
            let hex = b256::Base256::bytes_to_hex(&byte_array);
            io::stdout().write_all(&hex).unwrap();
            println!();
        }
        // raw -> b256 (default)
        (false, false, false) => {
            if trimmed.len() != 32 {
                eprintln!("error: expected 32 bytes, got {}", trimmed.len());
                std::process::exit(1);
            }
            let mut byte_array = [0u8; 32];
            byte_array.copy_from_slice(&trimmed[..32]);
            let encoded = b256::Base256::encode(&byte_array);
            for c in &encoded {
                print!("{}", c);
            }
            println!();
        }
        // invalid combination
        (true, true, true) => {
            eprintln!("error: -Xdx makes no sense (hex to hex through decode)");
            std::process::exit(1);
        }
    }
}
