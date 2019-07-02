use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Return a list of the prime factors for a natural number
fn trial_division(mut n: u64) -> Vec<u64> {
    let mut a = vec![];
    let mut f = 2;
    while n > 1 {
        if n % f == 0 {
            a.push(f);
            n /= f;
        } else {
            f += 1;
        }
    }
    a
}


#[pymodule]
fn primes(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "calculate")]
    fn calculate(py: Python, i: u64, n0: u64, n1: u64) -> PyResult<()> {
        py.allow_threads(|| {
            for n in n0..n1 {
                let a = trial_division(n);
                println!("Thread {}: {:?}", i, a);
            }
        });

        Ok(())
    }

    Ok(())
}

