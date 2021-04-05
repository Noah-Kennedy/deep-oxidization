pub struct Matrix<T, const R: usize, const C: usize> {
    inner: [[T; R]; C]
}

pub type RVec<T, const N: usize> = Matrix<T, 1, N>;

pub type CVec<T, const N: usize> = Matrix<T, N, 1>;