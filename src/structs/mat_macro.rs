#![macro_use]

#[macro_export]
macro_rules! mat {
    ( $n:ident, $h:expr, $w:expr ) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $n<T> where T: Copy {
            data: [T; $w * $h]
        }

        impl<T> Matrix<T> for $n<T>
            where T: Copy + Zero {
            
            #[inline]
            fn new() -> Self {
                $n {
                    data: [Zero::zero(); $w * $h]
                }
            }
            
            #[inline]
            fn new_filled(data: &[T]) -> Self {
                assert!(data.len() == $w * $h);

                let mut mat = Self::new(); {
                    let it = mat.iter_mut().zip(data);

                    for (mat_el, data_el) in it {
                        *mat_el = *data_el;
                    }
                }
                mat
            }
            
            #[inline]
            fn cols() -> usize {
                $w
            }
            
            #[inline]
            fn rows() -> usize {
                $h
            }
            
            #[inline]
            fn iter(&self) -> Iter<T> {
                Iter::new(
                    self.data.iter(),
                    $h,
                    $w
                )
            }
            
            #[inline]
            fn iter_mut(&mut self) -> IterMut<T> {
                IterMut::new(
                    self.data.iter_mut(),
                    $h,
                    $w
                )
            }
        }

        impl<T> Index<usize> for $n<T> where T: Copy {
            type Output = [T];
        
            #[inline]
            fn index<'a>(&'a self, row: usize) -> &'a [T] {
                &self.data[$w * row .. $w + $w * row]
            }
        }    

        impl<T> IndexMut<usize> for $n<T> where T: Copy {
            
            #[inline]
            fn index_mut<'a>(&'a mut self, row: usize) -> &'a mut [T] {
                &mut self.data[$w * row .. $w + $w * row]
            }
        }

        impl<T> Add for $n<T> where T: Copy + Zero + Add<Output=T> {
            type Output = Self;
            
            #[inline]
            fn add(self, rhs: Self) -> Self {
                let mut mat = Self::new(); {
                    let it = mat.data.iter_mut()
                        .zip(&self.data)
                        .zip(&rhs.data);

                    for ((res, first), second) in it {
                        *res = *first + *second;
                    }
                }
                mat
            }
        }

        impl<T> Sub for $n<T> where T: Copy + Zero + Sub<Output=T> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                let mut mat = Self::new(); {
                    let it = mat.data.iter_mut()
                        .zip(&self.data)
                        .zip(&rhs.data);

                    for ((res, first), second) in it {
                        *res = *first - *second;
                    }
                }
                mat
            }
        }
    };
    ( $n:ident, $s:expr ) => {
        mat!($n, $s, $s);

        impl<T> MatrixSquare<T> for $n<T> where T: Copy + NumCast + Zero   {

            #[inline]
            fn new_identity() -> Self {
                let mut mat = Self::new(); {
                    let it = mat.iter_mut()
                        .enumerate2d()
                        .filter(|&(i, j, _)| i == j);
                    
                    for (_, _, el) in it {
                        *el = cast(1).unwrap();
                    }
                }
                mat
            }
            
            #[inline]
            fn new_diag(data: &[T]) -> Self {
                assert!(data.len() == $s);

                let mut mat = Self::new(); {
                    let it = mat.iter_mut()
                        .enumerate2d()
                        .filter(|&(i, j, _)| i == j)
                        .zip(data);

                    for ((_, _, mat_el), data_el) in it {
                        *mat_el = *data_el;
                    }
                }
                mat
            }
        }
    }
}

#[macro_export]
macro_rules! mat_t {
    ( $m:ident, $res:ident ) => {
        impl<T> Transposable<$m<T>, $res<T>> for $m<T>
            where T: Copy + Zero, $res<T>: Matrix<T> {
            
            #[inline]
            fn t(&self) -> $res<T> {
                assert!(Self::rows() == $res::cols()
                    && Self::cols() == $res::rows());

                let mut mat = $res::new(); {
                    let it = mat.iter_mut().enumerate2d();

                    for (i, j, el) in it {
                        *el = self[j][i];
                    }
                }

                mat
            }
        }
    };
    ( $m:ident ) => {
        mat_t!($m, $m);
    }
}

#[macro_export]
macro_rules! mat_mul {
    ( $f:ident, $s:ident, $res:ident ) => {
        impl<T> Mul<$s<T>> for $f<T>
            where T: Copy + Zero + Mul<Output=T>,
                  $f<T>: Matrix<T>,
                  $s<T>: Matrix<T>,
                  $res<T>: Matrix<T> {
            type Output = $res<T>;

            #[inline]
            fn mul(self, rhs: $s<T>) -> $res<T> {
                assert!(Self::cols() == $s::rows()
                        && Self::rows() == $res::rows()
                        && $s::cols() == $res::cols());
                
                let mut mat = $res::new(); {
                    let it = mat.iter_mut().enumerate2d();

                    for (i, j, el) in it {
                        for k in 0..Self::cols() {
                            *el = *el + self[i][k] * rhs[k][j]
                        }
                    }
                }

                mat
            }
        }
    };
    ( $m:ident ) => {
        mat_mul!($m, $m, $m);
    }
}
