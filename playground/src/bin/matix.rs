use ndarray::arr2;

fn main() {
    for x in 0..4 {
        let eq = abba(x);
        println!("x: {}, equal: {}", x, eq);
    }
}

pub fn abba(x: i32) -> bool {
    let a = arr2(&[[1, x], [2, 3]]);
    let b = arr2(&[[1, 1], [1, 2]]);

    let ab = a.dot(&b);
    let ba = b.dot(&a);

    println!("a{}+b{}={}", a, b, ab);
    println!("b{}+a{}={}", b, a, ba);

    ab == ba
}

pub fn tst() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b = arr2(&[[7, 8], [9, 10], [11, 12]]);

    let prod = a.dot(&b);

    println!("{}+", a);
    println!("{}=", b);
    println!("{}", prod);
}
