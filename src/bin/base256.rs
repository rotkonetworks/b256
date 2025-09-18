use std::env;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("base256 - dense byte to unicode encoding");
        println!();
        println!("Usage:");
        println!("  base256        encode 32 bytes from stdin");
        println!("  base256 -d     decode 32 chars from stdin");
        println!();
        println!("Examples:");
        println!("  head -c 32 /dev/urandom | base256");
        println!("  echo 'encoded_string' | base256 -d | xxd");
        return;
    }

    let decode_mode = args.contains(&"-d".to_string());

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    if decode_mode {
        // decode mode
        let s = std::str::from_utf8(&input).unwrap().trim();
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 32 {
            let mut char_array = ['\0'; 32];
            char_array.copy_from_slice(&chars);
            if let Some(decoded) = b256::Base256::decode(&char_array) {
                io::stdout().write_all(&decoded).unwrap();
            } else {
                eprintln!("error: invalid character in input");
                std::process::exit(1);
            }
        } else {
            eprintln!("error: expected 32 characters, got {}", chars.len());
            std::process::exit(1);
        }
    } else if input.len() == 32 {
        // encode mode
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&input[0..32]);
        let encoded = b256::Base256::encode(&bytes);
        for c in &encoded {
            print!("{}", c);
        }
        println!();
    } else if !input.is_empty() {
        eprintln!("error: expected 32 bytes, got {}", input.len());
        std::process::exit(1);
    }
}
