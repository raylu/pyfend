use pyo3::pymodule;

#[pymodule]
mod pyfend {
	use pyo3::prelude::{PyResult, pyfunction};

	#[pyfunction]
	fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
		Ok((a + b).to_string())
	}
}
