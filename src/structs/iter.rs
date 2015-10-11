use iterator2d::Iterator2d;
use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};

pub struct Iter<'a, T: 'a> {
    iter: SliceIter<'a, T>,
    rows: usize,
    cols: usize
}

pub struct IterMut<'a, T: 'a> {
    iter: SliceIterMut<'a, T>,
    rows: usize,
    cols: usize
}

macro_rules! iterator2d {
    ( $name:ident, $iter:ident, $t:ty ) => {
        impl<'a, T> $name<'a, T> {

            #[inline]
            pub fn new(iter: $iter<'a, T>, rows: usize, cols: usize) -> Self {
                $name {
                    iter: iter,
                    rows: rows,
                    cols: cols
                }
            }
        }

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

iterator2d!(Iter, SliceIter, &'a T);
iterator2d!(IterMut, SliceIterMut, &'a mut T);
