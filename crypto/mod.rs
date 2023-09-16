use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Error, Key, Nonce,
};
use chacha20::ChaCha20;
use rand::Rng;
use ring::digest::{digest, SHA256};
use salsa20::{
    cipher::{KeyIvInit, StreamCipher, StreamCipherSeek},
    Salsa20,
};

use crate::JsonDatabseKMH;

pub fn generate_random_utf8(size: usize) -> Vec<u8> {
    let rand_string: String = rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(size)
        .collect();
    rand_string.as_bytes().to_vec()
}

pub fn encrypt_database_aes(db: &Vec<JsonDatabseKMH>, password: &String) -> Result<Vec<u8>, Error> {
    let password_hashed: [u8; 32] = digest(&SHA256, password.as_bytes())
        .as_ref()
        .try_into()
        .unwrap();

    let dbstr = serde_json::to_string(&db).expect("Invalid DB format");

    let aesgcm = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(&password_hashed));

    aesgcm.encrypt(Nonce::from_slice(&[0; 12]), dbstr.as_bytes())
}

pub fn decrypt_database_aes(db: Vec<u8>, password: &String) -> Result<Vec<u8>, Error> {
    let password_hashed: [u8; 32] = digest(&SHA256, password.as_bytes())
        .as_ref()
        .try_into()
        .unwrap();

    let aesgcm = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(&password_hashed));

    aesgcm.decrypt(Nonce::from_slice(&[0; 12]), db.as_slice())
}

pub fn encrypt_database_salsa20(db: &Vec<JsonDatabseKMH>, password: &String) -> Vec<u8> {
    let password_hashed: [u8; 32] = digest(&SHA256, password.as_bytes())
        .as_ref()
        .try_into()
        .unwrap();

    let mut db_bytes = serde_json::to_string(&db)
        .expect("Invalid DB format")
        .as_bytes()
        .to_vec();

    let mut salsa20 = Salsa20::new(
        &password_hashed.try_into().unwrap(),
        &[0; 8].try_into().unwrap(),
    );

    salsa20.apply_keystream(&mut db_bytes[..]);

    db_bytes
}

pub fn decrypt_database_salsa20(mut db: Vec<u8>, password: &String) -> Vec<u8> {
    let password_hashed: [u8; 32] = digest(&SHA256, password.as_bytes())
        .as_ref()
        .try_into()
        .unwrap();

    let mut salsa20 = Salsa20::new(
        &password_hashed.try_into().unwrap(),
        &[0; 8].try_into().unwrap(),
    );

    salsa20.seek(0u32);
    salsa20.apply_keystream(&mut db[..]);

    db
}

pub fn encrypt_database_chacha20(db: &Vec<JsonDatabseKMH>, password: &String) -> Vec<u8> {
    let password_hashed: [u8; 32] = digest(&SHA256, password.as_bytes())
        .as_ref()
        .try_into()
        .unwrap();

    let mut db_bytes = serde_json::to_string(&db)
        .expect("Invalid DB format")
        .as_bytes()
        .to_vec();

    let mut chacha20 = ChaCha20::new(
        &password_hashed.try_into().unwrap(),
        &[0; 12].try_into().unwrap(),
    );

    chacha20.apply_keystream(&mut db_bytes[..]);

    db_bytes
}

pub fn decrypt_database_chacha20(mut db: Vec<u8>, password: &String) -> Vec<u8> {
    let password_hashed: [u8; 32] = digest(&SHA256, password.as_bytes())
        .as_ref()
        .try_into()
        .unwrap();

    let mut chacha20 = ChaCha20::new(
        &password_hashed.try_into().unwrap(),
        &[0; 12].try_into().unwrap(),
    );

    chacha20.seek(0u32);
    chacha20.apply_keystream(&mut db[..]);

    db
}
