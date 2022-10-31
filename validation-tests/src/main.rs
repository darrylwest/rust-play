use std::borrow::Cow;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    email: Cow<'static, str>,
    #[validate(length(min = 0), custom = "validate_phone")]
    phone: Cow<'static, str>,
    #[validate(url)]
    site: Cow<'static, str>,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    first_name: Cow<'static, str>,
    #[validate(range(min = 18, max = 30))]
    age: u32,
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if phone.is_empty() {
        return Ok(());
    } 
    
    if phone.len() == 10 {
        return Ok(());
    }

    return Err(ValidationError::new("bad phone"));
}

// fn is_numeric(s: &str) -> Result<(), 

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxBadxXx" {
        return Err(ValidationError::new("a bad user name"));
    }

    Ok(())
}

fn main() {
    let sd = SignupData{
        email: Cow::from("dpw@raincitysoftware.com"),
        phone: Cow::from("5551236666"),
        site: Cow::from("https://raincitysoftware.com"),
        first_name: Cow::from("darryl"),
        age: 30,
    };

    match sd.validate() {
        Ok(_) => println!("{:?} is valid", sd),
        Err(e) => println!("{:?} error: {}", sd, e),
    }
}
