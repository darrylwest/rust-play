use anyhow::Result;

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

fn main() -> Result<()> {
    rand_letters();

    Ok(())
}
