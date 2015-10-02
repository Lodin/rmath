use iterator2d::Iterator2d;
use std::slice;
use std::ops::{ Add, Sub, Mul, Index, IndexMut };
use num::traits::{ cast, NumCast, Zero };

pub trait Matrix<T>
    where T: Copy + Add + Sub + Mul + NumCast + Zero {
    fn new() -> Self;
    fn new_filled(data: &[T]) -> Self;
    fn iter(&self) -> Iter<T>;
    fn iter_mut(&mut self) -> IterMut<T>;
}

pub trait MatrixSquare<T>
    where T: Copy + Add + Sub + Mul + NumCast + Zero   {
    fn new_identity() -> Self;
    fn new_diag(data: &[T]) -> Self;
}

pub trait Transposable<T, M>
    where T: Copy + Add + Sub + Mul + NumCast + Zero   {
    fn t(&self) -> M;
}

#[macro_export]
macro_rules! mat {
    ( $n:ident, $w:expr, $h:expr ) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $n<T>
            where T: Copy + Add + Sub + Mul + NumCast + Zero   {
            data: [T; $w * $h]
        }

        impl<T> Matrix<T> for $n<T>
            where T: Copy + Add + Sub + Mul + NumCast + Zero {
            
            #[inline]
            fn new() -> Self {
                $n {
                    data: [Zero::zero(); $w * $h]
                }
            }
            
            #[inline]
            fn new_filled(data: &[T]) -> Self {
                if data.len() != $w * $h {
                    panic!("Matrix size should be {}, not {}",
                        $w * $h, data.len());
                } 
                
                let mut mat = Self::new();
                {
                    let it = mat.iter_mut().zip(data);

                    for (mat_el, data_el) in it {
                        *mat_el = *data_el;
                    }
                }
                mat
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

        impl<T> Index<usize> for $n<T>
            where T:  Copy + Add + Sub + Mul + NumCast + Zero {
            type Output = [T];
        
            #[inline]
            fn index<'a>(&'a self, row: usize) -> &'a [T] {
                &self.data[$w * row .. $w + $w * row]
            }
        }    

        impl<T> IndexMut<usize> for $n<T>
            where T: Copy + Add + Sub + Mul + NumCast + Zero {
            
            #[inline]
            fn index_mut<'a>(&'a mut self, row: usize) -> &'a mut [T] {
                &mut self.data[$w * row .. $w + $w * row]
            }
        }
    };
    ( $n:ident, $s:expr ) => {
        mat!($n, $s, $s);

        impl<T> MatrixSquare<T> for $n<T>
            where T: Copy + Add + Sub + Mul + NumCast + Zero   {

            #[inline]
            fn new_identity() -> Self {
                let mut mat = Self::new();
                {
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
                if data.len() != $s {
                    panic!("Matrix diagonal length is {}, not {}",
                          $s, data.len())
                }

                let mut mat = Self::new();
                {
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

mat!(Matrix3x3, 3);

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
