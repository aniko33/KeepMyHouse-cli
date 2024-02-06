use std::process::exit;

use crate::Encryption;

pub fn encryption_str_to_enum(encryption: &String) -> Encryption{
    let encryption_type = match encryption.to_lowercase().as_str() {
        "aes256" => Encryption::AES256GCM,
        "salsa20" => Encryption::SALSA20,
        "chacha20" => Encryption::CHACHA20,
        _ => {
            eprintln!("This encryption don't exist :(");
            exit(1);
        }
    };

    return encryption_type;
}