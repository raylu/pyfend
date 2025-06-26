use pyo3::{
	Bound,
	prelude::{PyResult, pyfunction},
	pymodule,
	types::{PyModule, PyModuleMethods},
	wrap_pyfunction,
};

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
	Ok((a + b).to_string())
}

#[pymodule]
fn pyfend(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
	Ok(())
}
