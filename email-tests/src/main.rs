
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn read_creds() -> Credentials {
    // read from key store
    use std::env;
    use base64::decode;

    let b64 = env::var("EMAIL_CREDS").expect("should read creds from env");
    let plain = decode(&b64).expect("should decode the b64");
    let plain = String::from_utf8(plain).expect("should be string");

    // println!("{}", &plain);
    let v: Vec<&str> = plain.split(":").collect();

    let username = v[0].to_string();
    let password = v[1].to_string();

    Credentials::new(username, password)
}

// generate a 6 digit random number
fn generate_otp() -> u64 {
    let range = 100_000..1_000_000_u64;
    fastrand::u64(range)
}

fn main() {
    let from = "darryl.west<darryl.west@raincitysoftware.com>";
    let to = "<dpw500@raincitysoftware.com>";

    // todo - write the otp file
    let subject = "otp";
    let otp = generate_otp();
    let body = format!("{}", otp);

    println!("otp: {} to: {}", otp, to);

    let email = Message::builder()
        .from(from.parse().unwrap())
        .reply_to(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds = read_creds();

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_otp() {
        for _i in 0..10 {
            let otp = generate_otp();
            println!("{}", otp);
        }
    }
}
