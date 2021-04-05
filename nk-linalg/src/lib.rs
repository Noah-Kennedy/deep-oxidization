use std::ops::Mul;

pub struct Matrix<T, const R: usize, const C: usize> {
    inner: [[T; R]; C]
}

pub type RVec<T, const N: usize> = Matrix<T, 1, N>;

pub type CVec<T, const N: usize> = Matrix<T, N, 1>;

impl <T, const R: usize, const C: usize, const N: usize> Mul<Matrix<T, N, C>> for Matrix<T, R, N> {
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: Matrix<T, N, C>) -> Self::Output {
        todo!()
    }
}