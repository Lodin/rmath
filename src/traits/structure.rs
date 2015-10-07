use structs::mat::{ Iter, IterMut };

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
