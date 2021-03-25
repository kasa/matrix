use std::ops;

#[derive(Debug, Eq, PartialEq)]
pub enum LayoutScheme {
	RowMajor,
	ColMajor,
}

// Matrix trait
pub trait Matrix<T>:
	ops::Index<(usize, usize), Output = T>
	+ ops::IndexMut<(usize, usize), Output = T>
	+ ops::Index<usize, Output = [T]>
where
	T: num_traits::Zero,
{
	fn as_ptr(&self) -> *const T;
	fn as_mut_ptr(&mut self) -> *mut T;
	fn layout(&self) -> LayoutScheme;
	fn rows(&self) -> usize;
	fn cols(&self) -> usize;
	fn dim(&self) -> (usize, usize) {
		(self.rows(), self.cols())
	}
}

pub trait MatrixDense<T>: Matrix<T>
where
	T: num_traits::Zero,
{
	fn ld(&self) -> usize;
}

pub trait FixedMatrix<T, const M: usize, const N: usize>:
	ops::Index<(usize, usize), Output = T>
	+ ops::IndexMut<(usize, usize), Output = T>
	+ ops::Index<usize, Output = [T]>
where
	T: num_traits::Zero,
{
	fn as_ptr(&self) -> *const T;
	fn as_mut_ptr(&mut self) -> *mut T;
	fn layout(&self) -> LayoutScheme;
}
