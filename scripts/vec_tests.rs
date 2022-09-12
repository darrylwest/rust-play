#!/usr/bin/env rust-script
// cargo-deps: itertools

fn string_tests() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push('-');
    s1.push_str(s2);

    println!("s1 {}, s2 {}", s1, s2);

    for c in s1.chars() {
        print!("{} ", c);
    }
    println!("");

    for b in s1.bytes() {
        print!("{} ", b);
    }
    println!("");
}


fn main() {
    {
        let a = 5;
        let b = 3;

        println!("my test number: {}", a-- - --b);

    }

    let printr = |x| print!("{}, ", x);

    {
        // new scope
        let mut v = vec![1, 2, 3, 4, 5];

        let first = match v.get(0) {
            None => 1,
            Some(i) => *i,
        };

        println!("First: {:?}", first);

        v.push(6);
        v.insert(0, 7);

        println!("The list: {:?}", v);
        println!("Original First: {:?}", first);

        print!("reverse and filtered evens: ");
        v.into_iter()
            .rev()
            .filter(|x| x % 2 == 0)
            .for_each(|x| printr(x));
        println!("");
    }

    {
        // new scope
        println!("Modify a vector in place...");
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            print!("{}, ", i);
            *i += 50;
        }
        println!(" + 50 = ");
        v.into_iter().for_each(|x| printr(x));
        println!("");
    }

    string_tests();

    {
        let value = itertools::fold(&[1., 2., 3.], 0., |a, &b| f32::max(a, b));

        println!("max of 1, 2 3 is {}", value);

        let value = itertools::fold(&[1., 2., 3.], 0., |a, &b| a + b);

        println!("sum of 1, 2 3 is {}", value);
    }

}
