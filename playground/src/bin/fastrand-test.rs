use anyhow::Result;

fn rand_letters() {
    let v = vec!["a", "b", "c", "d", "e", "f"];

    for _ in 0..10 {
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
