extern crate rmath;
use rmath::types::matrix::{ Matrix, Matrix3x3 };

#[test]
fn test_matrix_basics() {
    let mat: Matrix3x3<i32> = Matrix3x3::new_filled(&[
         3, -1, 6,
         2,  1, 5,
        -3,  1, 0
    ]);

    assert_eq!(mat[2][0], -3);
}
