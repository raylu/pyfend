use pyo3::{create_exception, pymodule};

#[pymodule]
mod pyfend {
	use nanorand::Rng as _;
	use pyo3::{
		prelude::{PyResult, pyfunction},
		pyclass, pymethods,
	};

	#[pyfunction]
	fn evaluate(input: &str, context: &mut Context) -> PyResult<String> {
		let Context(fend_context) = context;
		fend_context.set_random_u32_fn(|| nanorand::WyRand::new().generate::<u32>());
		match fend_core::evaluate(input, fend_context) {
			Ok(output) => Ok(output.get_main_result().to_string()),
			Err(err_msg) => Err(FendError::new_err(err_msg)),
		}
	}

	#[pyclass]
	struct Context(fend_core::Context);
	#[pymethods]
	impl Context {
		#[new]
		fn new() -> Self {
			Context(fend_core::Context::new())
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
