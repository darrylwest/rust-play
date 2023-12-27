use anyhow::Result;
use std::iter::repeat_with;

fn rand_letters() {
    let v = ["0", "1", "2", "3", "4", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];

    for _ in 0..20 {
        let n = fastrand::usize(0..v.len());
        let f = v[n];

        if fastrand::bool() {
            print!("{}", f);
        } else {
            print!("{}", f.to_uppercase());
        }
    }

    println!();
}

fn rand_word(sz: usize) -> String {
    let s: String = repeat_with(fastrand::alphanumeric).take(sz).collect();
    println!("{}", s);

    s
}

fn rand_vec(sz: usize) -> Vec<u8> {
    let v: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(sz).collect();
    println!("{:?}", &v);

    let mut buf = String::new();
    for n in &v {
        buf.push_str(format!("{:02x}", n).as_str());
    }

    println!("{:?}", buf);

    v
}

fn main() -> Result<()> {
    rand_letters();
    rand_word(20);
    rand_vec(20);

    Ok(())
}
