use crate::{LayoutScheme, Matrix, MatrixDense};
use std::fmt;
use std::ops;

// Col major matrix
#[derive(Debug)]
pub struct MatrixColMajor<T>
where
	T: num_traits::Zero,
{
	m: usize,
	n: usize,
	d: Vec<T>,
}

impl<T> MatrixColMajor<T>
where
	T: num_traits::Zero + Clone,
{
	pub fn new(m: usize, n: usize) -> Self {
		return Self::from_vec(m, n, vec![num_traits::zero(); m * n]);
	}

	pub fn from_vec(m: usize, n: usize, d: Vec<T>) -> Self {
		assert_eq!(d.len(), m * n, "m*n != d.len()");
		return MatrixColMajor { m, n, d };
	}
}

impl<T> fmt::Display for MatrixColMajor<T>
where
	T: num_traits::Zero + Clone + fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for i in 0..self.m - 1 {
			for j in 0..self.n {
				write!(f, "\t{}", self.d[j * self.m + i])?
			}
			writeln!(f)?
		}
		for j in 0..self.n {
			write!(f, "\t{}", self.d[j * self.m + (self.m - 1)])?
		}
		Ok(())
	}
}

impl<T> Matrix<T> for MatrixColMajor<T>
where
	T: num_traits::Zero + Clone,
{
	fn as_ptr(&self) -> *const T {
		return self.d.as_ptr();
	}

	fn as_mut_ptr(&mut self) -> *mut T {
		return self.d.as_mut_ptr();
	}

	fn rows(&self) -> usize {
		return self.m;
	}

	fn cols(&self) -> usize {
		return self.n;
	}

	fn layout(&self) -> LayoutScheme {
		return LayoutScheme::ColMajor;
	}
}

impl<T> MatrixDense<T> for MatrixColMajor<T>
where
	T: num_traits::Zero + Clone,
{
	fn ld(&self) -> usize {
		return self.m;
	}
}

impl<T> ops::Index<(usize, usize)> for MatrixColMajor<T>
where
	T: num_traits::Zero + Clone,
{
	type Output = T;

	fn index(&self, m: (usize, usize)) -> &T {
		let (i, j) = m;

		return &(*self.d)[i + j * self.m];
	}
}

impl<T> ops::IndexMut<(usize, usize)> for MatrixColMajor<T>
where
	T: num_traits::Zero + Clone,
{
	fn index_mut(&mut self, m: (usize, usize)) -> &mut T {
		return ops::IndexMut::index_mut(&mut *self.d, m.0 + self.m * m.1);
	}
}

impl<T> ops::Index<usize> for MatrixColMajor<T>
where
	T: num_traits::Zero + Clone,
{
	type Output = [T];

	fn index(&self, n: usize) -> &[T] {
		return &(*self.d)[(n * self.m)..(n + 1) * self.m];
	}
}

impl<T> ops::IndexMut<usize> for MatrixColMajor<T>
where
	T: num_traits::Zero + Clone,
{
	fn index_mut(&mut self, n: usize) -> &mut [T] {
		return ops::IndexMut::index_mut(&mut *self.d, (n * self.m)..(n + 1) * self.m);
	}
}

#[cfg(test)]
mod tests {
	#![allow(non_snake_case)]
	use super::MatrixColMajor;
	use super::MatrixDense;
	use crate::Matrix;

	#[test]
	fn col_major() {
		let m = 3;
		let n = 2;

		let d = vec![0.0; m * n];
		let mut A = MatrixColMajor::from_vec(m, n, d);
		for i in 0..m {
			for j in 0..n {
				A[(i, j)] = (i * n + j) as f64;
			}
		}

		for i in 0..m {
			for j in 0..n {
				assert_eq!(A[(i, j)], (i * n + j) as f64);
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
		let (m, n) = (3, 2);
		let d = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
		let A = MatrixColMajor::from_vec(m, n, d);

		assert_eq!(1.0, A[0][0]);
		assert_eq!(2.0, A[0][1]);
		assert_eq!(3.0, A[0][2]);
		assert_eq!(4.0, A[1][0]);
		assert_eq!(5.0, A[1][1]);
		assert_eq!(6.0, A[1][2]);
	}

	#[test]
	fn col_ld_eq_3() {
		let (m, n) = (3, 2);
		let d = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
		let A = MatrixColMajor::from_vec(m, n, d);

		assert_eq!(3, A.ld());
	}
}
