use std::io::{self, Read, Write};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
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
            }
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
    }
}
