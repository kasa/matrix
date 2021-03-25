use crate::{FixedMatrix, LayoutScheme};
use std::fmt;
use std::ops;

// Col major matrix
#[derive(Debug)]
pub struct FixedMatrixColMajor<T, const M: usize, const N: usize>
where
	T: num_traits::Zero,
{
	d: Vec<T>,
}

impl<T, const M: usize, const N: usize> FixedMatrixColMajor<T, M, N>
where
	T: num_traits::Zero + Clone + fmt::Debug,
{
	pub fn new() -> Self {
		return Self::from_vec(vec![num_traits::zero(); M * N]);
	}

	pub fn from_vec(d: Vec<T>) -> Self {
		assert_eq!(d.len(), M * N, "m*n != d.len()");
		return FixedMatrixColMajor { d };
	}
}

impl<T, const M: usize, const N: usize> fmt::Display for FixedMatrixColMajor<T, M, N>
where
	T: num_traits::Zero + Clone + fmt::Display + fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for i in 0..M - 1 {
			for j in 0..N {
				write!(f, "\t{}", self.d[j * M + i])?
			}
			writeln!(f)?
		}
		for j in 0..N {
			write!(f, "\t{}", self.d[j * M + (M - 1)])?
		}
		Ok(())
	}
}

impl<T, const M: usize, const N: usize> FixedMatrix<T, M, N> for FixedMatrixColMajor<T, M, N>
where
	T: num_traits::Zero + Clone + fmt::Debug,
{
	fn as_ptr(&self) -> *const T {
		return self.d.as_ptr();
	}

	fn as_mut_ptr(&mut self) -> *mut T {
		return self.d.as_mut_ptr();
	}

	fn layout(&self) -> LayoutScheme {
		return LayoutScheme::ColMajor;
	}
}

impl<T, const M: usize, const N: usize> ops::Index<(usize, usize)> for FixedMatrixColMajor<T, M, N>
where
	T: num_traits::Zero + Clone + fmt::Debug,
{
	type Output = T;

	fn index(&self, m: (usize, usize)) -> &T {
		return &(*self.d)[m.0 + M * m.1];
	}
}

impl<T, const M: usize, const N: usize> ops::IndexMut<(usize, usize)>
	for FixedMatrixColMajor<T, M, N>
where
	T: num_traits::Zero + Clone + fmt::Debug,
{
	fn index_mut(&mut self, m: (usize, usize)) -> &mut T {
		return ops::IndexMut::index_mut(&mut *self.d, m.0 + M * m.1);
	}
}

impl<T, const M: usize, const N: usize> ops::Index<usize> for FixedMatrixColMajor<T, M, N>
where
	T: num_traits::Zero + Clone + fmt::Debug,
{
	type Output = [T];

	fn index(&self, n: usize) -> &[T] {
		return &(*self.d)[(n * M)..(n + 1) * M];
	}
}

impl<T, const M: usize, const N: usize> ops::IndexMut<usize> for FixedMatrixColMajor<T, M, N>
where
	T: num_traits::Zero + Clone + fmt::Debug,
{
	fn index_mut(&mut self, n: usize) -> &mut [T] {
		return ops::IndexMut::index_mut(&mut *self.d, (n * M)..(n + 1) * M);
	}
}

///////////////////////////// tests
#[cfg(test)]
mod tests {
	use super::FixedMatrixColMajor;
	use crate::FixedMatrix;

	#[test]
	fn col_major() {
		const M: usize = 3;
		const N: usize = 2;

		let d = vec![0.0; M * N];
		let mut A: FixedMatrixColMajor<_, M, N> = FixedMatrixColMajor::from_vec(d);
		for i in 0..M {
			for j in 0..N {
				A[(i, j)] = (i * N + j) as f64;
			}
		}

		for i in 0..M {
			for j in 0..N {
				assert_eq!(A[(i, j)], (i * N + j) as f64);
			}
		}

		let p = A.as_ptr();
		unsafe {
			assert_eq!(*p.offset((0) as isize), 0.0);
			assert_eq!(*p.offset((1) as isize), 2.0);
			assert_eq!(*p.offset((2) as isize), 4.0);
			assert_eq!(*p.offset((3) as isize), 1.0);
			assert_eq!(*p.offset((4) as isize), 3.0);
			assert_eq!(*p.offset((5) as isize), 5.0);
		}
	}

	#[test]
	fn index_access() {
		const M: usize = 3;
		const N: usize = 2;
		let d = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
		let A: FixedMatrixColMajor<_, M, N> = FixedMatrixColMajor::from_vec(d);

		assert_eq!(1.0, A[0][0]);
		assert_eq!(2.0, A[0][1]);
		assert_eq!(3.0, A[0][2]);
		assert_eq!(4.0, A[1][0]);
		assert_eq!(5.0, A[1][1]);
		assert_eq!(6.0, A[1][2]);
	}
}
