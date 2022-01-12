#!/usr/bin/env rust-script
// cargo-deps: num-bigint

use num_bigint::BigUint;

fn main() {
    let r = 2755320831786064189343839u128;
    let x = BigUint::from(r);
    let y = 17031828491509303443154495691094081u128;
    let z = BigUint::parse_bytes(b"13zqq5d92wbvyagg8aof4rl7ysfymqj2", 36).unwrap();
    let zz = BigUint::to_str_radix(&z, 36);
    let max = u128::MAX;
    
    println!("{:?} {:?} {:?} {}", y, x, z, zz);
    println!("{}", max);
    
    let mx = BigUint::from(max);
    println!("{}", BigUint::to_str_radix(&mx, 36));
    
    let bz = BigUint::parse_bytes(b"z5lxx1zz5pnorynqglhzmsp33", 36).unwrap();
    println!("{}", bz);
}
