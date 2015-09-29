
pub trait Matrix<T>
    where T: Add + Sub + Mul {
    fn new() -> Self;
    fn new_identity() -> Self;
    fn new_filled(data: &[T]) -> Self;
}

pub trait MatrixSquare<T>
    where T: Add + Sub + Mul {
    fn new_identity() -> Self;
    fn new_diag(data: &[T]) -> Self;
}

pub trait Transposable<T, M>
    where T: Add + Sub + Mul {
    fn t(&self) -> M;
}

#[macro_export]
macro_rules! mat {
    ( $n:ident, $s:ident ) => {
        mat!($n, $s, $s);

        impl<T> MatrixSquare<T> for $n {
            fn new_identity() -> Self {
                let mut mat = Self::new();
                let mut it = mat.iter_mut()
                    .enumerate2d()
                    .filter(|i, j, el| i == j);
                
                for (i, j, el) in it {
                    *el = 1;
                }
                mat
            }
            
            fn new_diag(data: &[T]) {
                if data.len() != $s {
                    panic!("Matrix diagonal length is {}, not {}",
                          $s, data.len())
                }

                let mut mat = Self::new();
                let mut it = mat.iter_mut()
                    .enumerate2d()
                    .filter(|i, j, el| i == j)
                    .zip(&data);

                for (i, j, (mat_el, data_el)) in it {
                    *mat_el = data_el;
                }
                mat
            }
        }
    }
    ( $n:ident, $w:ident, $h:ident ) => {
        pub struct $n<T> {
            data: [T; $w * $h]
        }

        impl<T> Matrix<T> for $n {
            fn new() -> Self {
                $n {
                    data: [0; $w * $h]
                }
            }

            
            fn new_filled(data: &[T]) -> Self {
                if data.len() != $w * $h {
                    panic!("Matrix size should be {}, not {}",
                           $w * $h, data.len());
                } 
                
                let mut mat = Self::new();
                let mut it = mat.iter_mut().zip(data);

                for ((mat_el, data_el)) in it {
                    *mat_el = data_el;
                }

                mat
            }
        }
    }
}
