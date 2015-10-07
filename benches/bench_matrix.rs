#![feature(test)]

extern crate test;
extern crate rmath;
use test::Bencher;
use rmath::types::matrix::{ Matrix, Mat3 };

pub fn create_matrix() -> Mat3<i32> {
    let m = Mat3::new_filled(&[
        3, -1, 6,
        2,  1, 5,
       -3,  1, 0
    ]);

    let n = m + m;

    n
}

#[bench]
fn bench_create_matrix(b: &mut Bencher) {
    b.iter(|| {
        create_matrix();
    });
}
