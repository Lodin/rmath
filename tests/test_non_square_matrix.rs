extern crate rmath;
use rmath::traits::structure::{Matrix, Transposable};
use rmath::structs::mat::Mat2;
use rmath::structs::mat_nsq::Mat2x3;

#[inline]
fn new_matrix() -> Mat2x3<i32> {
    Mat2x3::<i32>::new_filled(&[
        -3, 2,  4,
         0, 1, -2
    ])
}

#[test]
fn test_matrix_nsq_creation() {
    let mat = new_matrix();

    assert_eq!(mat[1][2], -2);
}

#[test]
fn test_matrix_nsq_transposition() {
    let mat = new_matrix();
    let mat_t = mat.t();

    assert_eq!(mat[0][2], mat_t[2][0]);
    assert_eq!(mat[1][1], mat_t[1][1]);
    assert_eq!(mat[1][2], mat_t[2][1]);
}

#[test]
fn test_matrix_nsq_add_sub() {
    let mat = new_matrix();
    
    let mat_add_eq = Mat2x3::<i32>::new_filled(&[
        -6, 4,  8,
         0, 2, -4
    ]);

    let mat_sub_eq = Mat2x3::<i32>::new();

    assert_eq!(mat + mat, mat_add_eq);
    assert_eq!(mat - mat, mat_sub_eq);
}

#[test]
fn test_matrix_nsq_mul() {
    let mat = new_matrix();
    let mat_t = mat.t();

    let mat_mul_eq = Mat2::<i32>::new_filled(&[
         29, -6,
        -6,   5
    ]);

    assert_eq!(mat * mat_t, mat_mul_eq);
}
