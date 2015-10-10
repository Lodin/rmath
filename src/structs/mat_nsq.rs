use iter::{ Iter, IterMut };
use mat::{ Mat2, Mat3, Mat4, Mat5, Mat6 };
use traits::structure::{ Matrix, Transposable };
use iterator2d::Iterator2d;
use std::ops::{ Add, Sub, Mul, Index, IndexMut };
use num::traits::{ Zero, NumCast, cast };

mat!(Mat2x3, 2, 3);
mat!(Mat2x4, 2, 4);
mat!(Mat2x5, 2, 5);
mat!(Mat2x6, 2, 6);

mat!(Mat3x2, 3, 2);
mat!(Mat3x4, 3, 4);
mat!(Mat3x5, 3, 5);
mat!(Mat3x6, 3, 6);

mat!(Mat4x2, 4, 2);
mat!(Mat4x3, 4, 3);
mat!(Mat4x5, 4, 5);
mat!(Mat4x6, 4, 6);

mat!(Mat5x2, 5, 2);
mat!(Mat5x3, 5, 3);
mat!(Mat5x4, 5, 4);
mat!(Mat5x6, 5, 6);

mat!(Mat6x2, 6, 2);
mat!(Mat6x3, 6, 3);
mat!(Mat6x4, 6, 4);
mat!(Mat6x5, 6, 5);

mat_t!(Mat2x3, Mat3x2);
mat_t!(Mat2x4, Mat4x2);
mat_t!(Mat2x5, Mat5x2);
mat_t!(Mat2x6, Mat6x2);

mat_t!(Mat3x2, Mat2x3);
mat_t!(Mat3x4, Mat4x3);
mat_t!(Mat3x5, Mat5x3);
mat_t!(Mat3x6, Mat6x3);

mat_t!(Mat4x2, Mat2x4);
mat_t!(Mat4x3, Mat3x4);
mat_t!(Mat4x5, Mat5x4);
mat_t!(Mat4x6, Mat6x4);

mat_t!(Mat5x2, Mat2x5);
mat_t!(Mat5x3, Mat3x5);
mat_t!(Mat5x4, Mat4x5);
mat_t!(Mat5x6, Mat6x5);

mat_t!(Mat6x2, Mat2x6);
mat_t!(Mat6x3, Mat3x6);
mat_t!(Mat6x4, Mat4x6);
mat_t!(Mat6x5, Mat5x6);

mat_mul!(Mat2, Mat2x3, Mat2x3);
mat_mul!(Mat2, Mat2x4, Mat2x4);
mat_mul!(Mat2, Mat2x5, Mat2x5);
mat_mul!(Mat2, Mat2x6, Mat2x6);

mat_mul!(Mat2x3, Mat3x2, Mat2);
mat_mul!(Mat2x3, Mat3,   Mat2x3);
mat_mul!(Mat2x3, Mat3x4, Mat2x4);
mat_mul!(Mat2x3, Mat3x5, Mat2x5);
mat_mul!(Mat2x3, Mat3x6, Mat2x6);

mat_mul!(Mat2x4, Mat4x2, Mat2);
mat_mul!(Mat2x4, Mat4x3, Mat2x3);
mat_mul!(Mat2x4, Mat4,   Mat2x4);
mat_mul!(Mat2x4, Mat4x5, Mat2x5);
mat_mul!(Mat2x4, Mat4x6, Mat2x6);

mat_mul!(Mat2x5, Mat5x2, Mat2);
mat_mul!(Mat2x5, Mat5x3, Mat2x3);
mat_mul!(Mat2x5, Mat5x4, Mat2x4);
mat_mul!(Mat2x5, Mat5,   Mat2x5);
mat_mul!(Mat2x5, Mat5x6, Mat2x6);

mat_mul!(Mat2x6, Mat6x2, Mat2);
mat_mul!(Mat2x6, Mat6x3, Mat2x3);
mat_mul!(Mat2x6, Mat6x4, Mat2x4);
mat_mul!(Mat2x6, Mat6x5, Mat2x5);
mat_mul!(Mat2x6, Mat6,   Mat2x6);


mat_mul!(Mat3x2, Mat2, Mat3x2);
mat_mul!(Mat3x2, Mat2x3, Mat3);
mat_mul!(Mat3x2, Mat2x4, Mat3x4);
mat_mul!(Mat3x2, Mat2x5, Mat3x5);
mat_mul!(Mat3x2, Mat2x6, Mat3x6);

mat_mul!(Mat3, Mat3x2, Mat3x2);
mat_mul!(Mat3, Mat3x4, Mat3x4);
mat_mul!(Mat3, Mat3x5, Mat3x5);
mat_mul!(Mat3, Mat3x6, Mat3x6);

mat_mul!(Mat3x4, Mat4x2, Mat3x2);
mat_mul!(Mat3x4, Mat4x3, Mat3);
mat_mul!(Mat3x4, Mat4,   Mat3x4);
mat_mul!(Mat3x4, Mat4x5, Mat3x5);
mat_mul!(Mat3x4, Mat4x6, Mat3x6);

mat_mul!(Mat3x5, Mat5x2, Mat3x2);
mat_mul!(Mat3x5, Mat5x3, Mat3);
mat_mul!(Mat3x5, Mat5x4, Mat3x4);
mat_mul!(Mat3x5, Mat5,   Mat3x5);
mat_mul!(Mat3x5, Mat5x6, Mat3x6);

mat_mul!(Mat3x6, Mat6x2, Mat3x2);
mat_mul!(Mat3x6, Mat6x3, Mat3);
mat_mul!(Mat3x6, Mat6x4, Mat3x4);
mat_mul!(Mat3x6, Mat6x5, Mat3x5);
mat_mul!(Mat3x6, Mat6,   Mat3x6);


mat_mul!(Mat4x2, Mat2,   Mat4x2);
mat_mul!(Mat4x2, Mat2x3, Mat4x3);
mat_mul!(Mat4x2, Mat2x4, Mat4);
mat_mul!(Mat4x2, Mat2x5, Mat4x5);
mat_mul!(Mat4x2, Mat2x6, Mat4x6);

mat_mul!(Mat4x3, Mat3x2, Mat4x2);
mat_mul!(Mat4x3, Mat3,   Mat4x3);
mat_mul!(Mat4x3, Mat3x4, Mat4);
mat_mul!(Mat4x3, Mat3x5, Mat4x5);
mat_mul!(Mat4x3, Mat3x6, Mat4x6);

mat_mul!(Mat4, Mat4x2, Mat4x2);
mat_mul!(Mat4, Mat4x3, Mat4x3);
mat_mul!(Mat4, Mat4x5, Mat4x5);
mat_mul!(Mat4, Mat4x6, Mat4x6);

mat_mul!(Mat4x5, Mat5x2, Mat4x2);
mat_mul!(Mat4x5, Mat5x3, Mat4x3);
mat_mul!(Mat4x5, Mat5x4, Mat4);
mat_mul!(Mat4x5, Mat5,   Mat4x5);
mat_mul!(Mat4x5, Mat5x6, Mat4x6);

mat_mul!(Mat4x6, Mat6x2, Mat4x2);
mat_mul!(Mat4x6, Mat6x3, Mat4x3);
mat_mul!(Mat4x6, Mat6x4, Mat4);
mat_mul!(Mat4x6, Mat6x5, Mat4x5);
mat_mul!(Mat4x6, Mat6,   Mat4x6);


mat_mul!(Mat5x2, Mat2,   Mat5x2);
mat_mul!(Mat5x2, Mat2x3, Mat5x3);
mat_mul!(Mat5x2, Mat2x4, Mat5x4);
mat_mul!(Mat5x2, Mat2x5, Mat5);
mat_mul!(Mat5x2, Mat2x6, Mat5x6);

mat_mul!(Mat5x3, Mat3x2, Mat5x2);
mat_mul!(Mat5x3, Mat3,   Mat5x3);
mat_mul!(Mat5x3, Mat3x4, Mat5x4);
mat_mul!(Mat5x3, Mat3x5, Mat5);
mat_mul!(Mat5x3, Mat3x6, Mat5x6);

mat_mul!(Mat5x4, Mat4x2, Mat5x2);
mat_mul!(Mat5x4, Mat4x3, Mat5x3);
mat_mul!(Mat5x4, Mat4,   Mat5x4);
mat_mul!(Mat5x4, Mat4x5, Mat5);
mat_mul!(Mat5x4, Mat4x6, Mat5x6);

mat_mul!(Mat5, Mat5x2, Mat5x2);
mat_mul!(Mat5, Mat5x3, Mat5x3);
mat_mul!(Mat5, Mat5x4, Mat5x4);
mat_mul!(Mat5, Mat5x6, Mat5x6);

mat_mul!(Mat5x6, Mat6x2, Mat5x2);
mat_mul!(Mat5x6, Mat6x3, Mat5x3);
mat_mul!(Mat5x6, Mat6x4, Mat5x4);
mat_mul!(Mat5x6, Mat6x5, Mat5);
mat_mul!(Mat5x6, Mat6,   Mat5x6);


mat_mul!(Mat6x2, Mat2,   Mat6x2);
mat_mul!(Mat6x2, Mat2x3, Mat6x3);
mat_mul!(Mat6x2, Mat2x4, Mat6x4);
mat_mul!(Mat6x2, Mat2x5, Mat6x5);
mat_mul!(Mat6x2, Mat2x6, Mat6);

mat_mul!(Mat6x3, Mat3x2, Mat6x2);
mat_mul!(Mat6x3, Mat3,   Mat6x3);
mat_mul!(Mat6x3, Mat3x4, Mat6x4);
mat_mul!(Mat6x3, Mat3x5, Mat6x5);
mat_mul!(Mat6x3, Mat3x6, Mat6);

mat_mul!(Mat6x4, Mat4x2, Mat6x2);
mat_mul!(Mat6x4, Mat4x3, Mat6x3);
mat_mul!(Mat6x4, Mat4,   Mat6x4);
mat_mul!(Mat6x4, Mat4x5, Mat6x5);
mat_mul!(Mat6x4, Mat4x6, Mat6);

mat_mul!(Mat6x5, Mat5x2, Mat6x2);
mat_mul!(Mat6x5, Mat5x3, Mat6x3);
mat_mul!(Mat6x5, Mat5x4, Mat6x4);
mat_mul!(Mat6x5, Mat5,   Mat6x5);
mat_mul!(Mat6x5, Mat5x6, Mat6);

mat_mul!(Mat6, Mat6x2, Mat6x2);
mat_mul!(Mat6, Mat6x3, Mat6x3);
mat_mul!(Mat6, Mat6x4, Mat6x4);
mat_mul!(Mat6, Mat6x5, Mat6x5);
