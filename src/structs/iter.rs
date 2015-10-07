use std::slice::{ Iter as SliceIter, IterMut as SliceIterMut };

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

iterator2d!(Iter, SliceIter, &'a T);
iterator2d!(IterMut, SliceIterMut, &'a mut T);
