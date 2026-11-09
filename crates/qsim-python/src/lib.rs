mod errors;
mod simulator;

use pyo3::prelude::*;

#[pymodule]
fn _core(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(simulator::run, module)?)?;
    Ok(())
}
