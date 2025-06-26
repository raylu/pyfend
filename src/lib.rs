use pyo3::pymodule;

#[pymodule]
mod pyfend {
	use pyo3::prelude::{PyResult, pyfunction};

	#[pyfunction]
	fn evaluate(input: &str) -> PyResult<String> {
		let mut context = fend_core::Context::new();
		match fend_core::evaluate(input, &mut context) {
			Ok(output) => Ok(output.get_main_result().to_string()),
			Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
		}
	}
}
