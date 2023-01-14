use anyhow::Result;

fn main() -> Result<()> {
    let v = vec!["a", "b", "c", "d", "e", "f"];

    for _ in 0..10 {
        let n = fastrand::usize(0..v.len());
        let f = v[n];

        print!("{}", f);
    }

    println!("");

    Ok(())
}
