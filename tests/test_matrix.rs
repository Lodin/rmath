extern crate rmath;
use rmath::types::matrix::{ Matrix, MatrixSquare, Matrix3x3 };

#[test]
fn test_matrix_creation_simple() {
    let mat = Matrix3x3::<i32>::new_filled(&[
         3, -1, 6,
         2,  1, 5,
        -3,  1, 0
    ]);

    assert_eq!(mat[2][0], -3);
}

#[test]
fn test_matrix_creation_square() {
    let mat = Matrix3x3::<i32>::new_identity();

    assert_eq!(mat[0][0], 1);
    assert_eq!(mat[0][1], 0);
    assert_eq!(mat[1][0], 0);
    assert_eq!(mat[1][1], 1);

    let mat2 = Matrix3x3::<i32>::new_diag(&[1, 2, 3]);

    assert_eq!(mat2[0][0], 1);
    assert_eq!(mat2[0][1], 0);
    assert_eq!(mat2[1][0], 0);
    assert_eq!(mat2[1][1], 2);
}
