
use anyhow::Result;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2
};

fn make_hash(password: String) -> Result<String> {
    let pw = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    println!("{}", salt);

    let hash = argon2.hash_password(pw, &salt)?.to_string();

    println!("hash {}", hash);

    Ok(hash)
}

fn verify_password(password: String, hash: String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hash)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();

    Ok(result)
}


fn main() -> Result<()> {
    let pw = String::from("my-password243");
    let hash = make_hash(pw.clone())?;

    // happy path
    let result = verify_password(pw.to_string(), hash.clone())?;
    println!("shoud be valid, true: {}", result);

    // test the failure
    let pw = String::from("anything-but");
    
    let result = verify_password(pw.to_string(), hash)?;
    println!("should fail on bad password, i.e. shoud be false: {}", result);

    Ok(())
}
