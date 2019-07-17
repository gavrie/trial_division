use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod trial_div;
use trial_div::trial_div;

#[pyfunction]
fn trial_division(n: u64) -> PyResult<Vec<u64>> {
    Ok(trial_div(n))
}

#[pyfunction]
fn trial_division_no_gil(py: Python, n: u64) -> PyResult<Vec<u64>> {
    Ok(py.allow_threads(|| {
        trial_div(n)
    }))
}


#[pymodule]
fn primes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(trial_division))?;
    m.add_wrapped(wrap_pyfunction!(trial_division_no_gil))?;
    Ok(())
}
