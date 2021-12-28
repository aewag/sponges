#![feature(test)]
extern crate keccak;
extern crate test;

#[bench]
fn f200(b: &mut test::Bencher) {
    let mut data = [0u8; 25];
    b.iter(|| keccak::f200(&mut data));
}

#[bench]
fn f400(b: &mut test::Bencher) {
    let mut data = [0u16; 25];
    b.iter(|| keccak::f400(&mut data));
}

#[bench]
fn f800(b: &mut test::Bencher) {
    let mut data = [0u32; 25];
    b.iter(|| keccak::f800(&mut data));
}

#[bench]
fn f1600(b: &mut test::Bencher) {
    let mut data = [0u64; 25];
    b.iter(|| keccak::f1600(&mut data));
}
