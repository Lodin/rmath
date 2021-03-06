use iter::{Iter, IterMut}; 
use traits::structure::{Matrix, MatrixSquare, Transposable};
use iterator2d::Iterator2d;
use std::ops::{Add, Sub, Mul, Index, IndexMut};
use num::traits::{Zero, NumCast, cast};
use rand::{Rand, Rng};

mat!(Mat1, 1);
mat!(Mat2, 2);
mat!(Mat3, 3);
mat!(Mat4, 4);
mat!(Mat5, 5);
//mat!(Mat6, 6);

mat_t!(Mat1);
mat_t!(Mat2);
mat_t!(Mat3);
mat_t!(Mat4);
mat_t!(Mat5);
//mat_t!(Mat6);

mat_mul!(Mat1);
mat_mul!(Mat2);
mat_mul!(Mat3);
mat_mul!(Mat4);
mat_mul!(Mat5);
//mat_mul!(Mat6);
