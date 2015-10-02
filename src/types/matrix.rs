use iterator2d::Iterator2d;
use std::slice;
use std::ops::{ Add, Sub, Mul, Index, IndexMut };
use num::traits::{ cast, NumCast, Zero };

pub trait Matrix<T> where T: Copy {
    fn new() -> Self;
    fn new_filled(data: &[T]) -> Self;

    fn cols() -> usize;
    fn rows() -> usize;

    fn iter(&self) -> Iter<T>;
    fn iter_mut(&mut self) -> IterMut<T>;
}

pub trait MatrixSquare<T> where T: Copy {
    fn new_identity() -> Self;
    fn new_diag(data: &[T]) -> Self;
}

pub trait Transposable<T, M> where T: Copy {
    fn t(&self) -> M;
}

#[macro_export]
macro_rules! mat {
    ( $n:ident, $w:expr, $h:expr ) => {
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
                Iter {
                    iter: self.data.iter(),
                    rows: $h,
                    cols: $w
                }
            }
            
            #[inline]
            fn iter_mut(&mut self) -> IterMut<T> {
                IterMut {
                    iter: self.data.iter_mut(),
                    rows: $h,
                    cols: $w
                }
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
macro_rules! mat_mul {
    ( $f:ident, $s:ident, $res:ident ) => {
        impl<T> Mul for $f<T>
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
                        for k in (0..Self::cols()) {
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

mat!(Matrix1x1, 1);
mat!(Matrix2x2, 2);
mat!(Matrix3x3, 3);
mat!(Matrix4x4, 4);

mat_mul!(Matrix1x1);
mat_mul!(Matrix2x2);
mat_mul!(Matrix3x3);
mat_mul!(Matrix4x4);

macro_rules! iterator2d {
    ( struct $name:ident, struct $iter:path, $t:ty ) => {
        
        impl<'a, T> Iterator2d for $name<'a, T> {
    
            #[inline]
            fn rows(&self) -> usize {
                self.rows
            }

            #[inline]
            fn cols(&self) -> usize {
                self.cols
            }
        }

        impl<'a, T> Iterator for $name<'a, T> {
            type Item = $t;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.iter.nth(n)
            }

            #[inline]
            fn count(self) -> usize {
                self.iter.count()
            }
        } 
    }
}

pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
    rows: usize,
    cols: usize
}

iterator2d!(struct Iter, struct slice::Iter, &'a T);

pub struct IterMut<'a, T: 'a> {
    iter: slice::IterMut<'a, T>,
    rows: usize,
    cols: usize
}

iterator2d!(struct IterMut, struct slice::IterMut, &'a mut T);
