extern crate rmath;
use rmath::traits::structure::{Matrix, MatrixSquare, Transposable};
use rmath::structs::mat::{Mat3};

#[inline]
fn new_matrix() -> Mat3<i32> {
    Mat3::<i32>::new_filled(&[
         3, -1, 6,
         2,  1, 5,
        -3,  1, 0
    ])
}

#[test]
fn test_matrix_creation_simple() {
    let mat = new_matrix();

    assert_eq!(mat[2][0], -3);
}

#[test]
fn test_matrix_creation_square() {
    let mat = Mat3::<i32>::new_identity();

    assert_eq!(mat[0][0], 1);
    assert_eq!(mat[0][1], 0);
    assert_eq!(mat[1][0], 0);
    assert_eq!(mat[1][1], 1);

    let mat2 = Mat3::<i32>::new_diag(&[1, 2, 3]);

    assert_eq!(mat2[0][0], 1);
    assert_eq!(mat2[0][1], 0);
    assert_eq!(mat2[1][0], 0);
    assert_eq!(mat2[1][1], 2);
}

#[test]
fn test_matrix_transposition() {
    let mat = new_matrix();
    let mat_t = mat.t();

    assert_eq!(mat[0][1], mat_t[1][0]);
    assert_eq!(mat[2][1], mat_t[1][2]);
}

#[test]
fn test_matrix_add_sub() {
    let mat = new_matrix();

    let mat_add_eq = Mat3::<i32>::new_filled(&[
         6, -2, 12,
         4,  2, 10,
        -6,  2,  0 
    ]); 

    let mat_sub_eq = Mat3::<i32>::new();

    assert_eq!(mat + mat, mat_add_eq);
    assert_eq!(mat - mat, mat_sub_eq);
}

#[test]
fn test_matrix_mul() {
    let mat = new_matrix(); 

    let mat_mul_eq = Mat3::<i32>::new_filled(&[
        -11, 2,  13,
         -7, 4,  17,
         -7, 4, -13
    ]);

    assert_eq!(mat * mat, mat_mul_eq);
}
