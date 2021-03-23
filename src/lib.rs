#![allow(non_snake_case)]

#[cfg(nightly)]
mod const_matrix_col;

mod matrix_col;
mod matrix_row;
mod traits;

pub use matrix_col::MatrixColMajor;
pub use matrix_row::MatrixRowMajor;
pub use traits::LayoutScheme;
pub use traits::Matrix;
pub use traits::MatrixDense;
