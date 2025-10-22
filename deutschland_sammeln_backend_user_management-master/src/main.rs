use argon2::password_hash::{SaltString};
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct Input{
    password: String
}

fn main() {
    let mut input = String::new();
    File::open("input.toml").unwrap().read_to_string(&mut input).unwrap();

    let input: Input = toml::from_str(&input).unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(input.password.as_bytes(), &salt).unwrap();

    let hash_string = hash.serialize();

    println!("{}", &hash_string);
}
