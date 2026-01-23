pub mod core;
pub mod python_module;
pub mod r_module;

use pyo3::prelude::*;
#[pymodule]
fn glia_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<python_module::PyPushResult>()?;
    m.add_function(wrap_pyfunction!(python_module::push_telemetry, m)?)?;
    Ok(())
}
