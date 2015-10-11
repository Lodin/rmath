#![feature(test)]

extern crate test;
extern crate rand;
extern crate rmath;

use rand::{IsaacRng, Rng};
use test::Bencher;
use rmath::{Mat2, Mat3, Mat4};
use std::ops::{Add, Sub, Mul, Div};

#[path="common/macros.rs"]
mod macros;

bench_binop!(bench_mat2_mul, Mat2<f32>, Mat2<f32>, mul);
bench_binop!(bench_mat3_mul, Mat3<f32>, Mat3<f32>, mul);
bench_binop!(bench_mat4_mul, Mat4<f32>, Mat4<f32>, mul);

bench_binop!(bench_mat2_add, Mat2<f32>, Mat2<f32>, add);
bench_binop!(bench_mat3_add, Mat3<f32>, Mat3<f32>, add);
bench_binop!(bench_mat4_add, Mat4<f32>, Mat4<f32>, add);

bench_binop!(bench_mat2_sub, Mat2<f32>, Mat2<f32>, sub);
bench_binop!(bench_mat3_sub, Mat3<f32>, Mat3<f32>, sub);
bench_binop!(bench_mat4_sub, Mat4<f32>, Mat4<f32>, sub);
