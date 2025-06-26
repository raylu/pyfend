use pyo3::{create_exception, pymodule};

#[pymodule]
mod pyfend {
	use pyo3::prelude::{PyResult, pyfunction};

	#[pyfunction]
	fn evaluate(input: &str) -> PyResult<String> {
		let mut context = fend_core::Context::new();
		match fend_core::evaluate(input, &mut context) {
			Ok(output) => Ok(output.get_main_result().to_string()),
			Err(err_msg) => Err(FendError::new_err(err_msg)),
		}
	}

	#[pymodule_export]
	use super::FendError;
}

create_exception!(
	pyfend,
	FendError,
	pyo3::exceptions::PyException,
	"fend evaluation error"
);
