use argon2::password_hash::{SaltString};
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};

fn main() {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password("".as_bytes(), &salt).unwrap();

    let hash_string = hash.serialize();

    println!("{}", &hash_string);
}
