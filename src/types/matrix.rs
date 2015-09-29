use iterator2d::Iterator2d;
use std::slice;

pub trait Matrix<T>
    where T: Add + Sub + Mul {
    fn new() -> Self;
    fn new_identity() -> Self;
    fn new_filled(data: &[T]) -> Self;
    fn iter(&self) -> Iter<T>;
    fn iter_mut(&mut self) -> IterMut<T>
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
    ( $n:ident, $w:ident, $h:ident ) => {
        pub struct $n<T> {
            data: [T; $w * $h]
        }

        impl<T> Matrix<T> for $n<T> {
            
            #[inline]
            fn new() -> Self {
                $n {
                    data: [0; $w * $h]
                }
            }
            
            #[inline]
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
            
            #[inline]
            fn iter(&self) -> Iter<'a, T> {
                Iter {
                    iter: self.data.iter(),
                    rows: $h,
                    cols: $w
                }
            }
            
            #[inline]
            fn iter_mut(&mut self) -> IterMut<'a, T> {
                IterMut {
                    iter: self.data.iter_mut(),
                    rows: $h,
                    cols: $w
                }
            }
        }

    };
    ( $n:ident, $s:ident ) => {
        mat!($n, $s, $s);

        impl<T> MatrixSquare<T> for $n<T> {

            #[inline]
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
            
            #[inline]
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
}

macro_rules! iterator2d {
    ( struct $name:ident, struct $iter:ident, $t:ty ) => {
        impl<T> $name<T> {
            
            #[inline]
            pub fn new(iter: $iter<T>, rows: usize, cols: usize)
                -> $name<T> {
                $name {
                    iter: iter,
                    rows: rows,
                    cols: cols
                }
            }
        }

        impl<T> Iterator2d for $name<T> {
    
            #[inline]
            fn rows(&self) -> usize {
                self.rows
            }

            #[inline]
            fn cols(&self) -> usize {
                self.cols
            }
        }

        impl<T> Iterator for $name<T> {
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
