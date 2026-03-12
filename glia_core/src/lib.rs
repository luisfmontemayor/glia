pub mod core;

#[cfg(feature = "python")]
pub mod python_module;

#[cfg(feature = "r")]
pub mod r_module;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::python_module::*;

#[cfg(feature = "python")]
#[pymodule]
#[pyo3(name = "glia_core")]
fn glia_core_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFlushSummary>()?;
    m.add_function(wrap_pyfunction!(queue_telemetry, m)?)?;
    m.add_function(wrap_pyfunction!(flush_queue, m)?)?;
    m.add_function(wrap_pyfunction!(trigger_panic, m)?)?;
    Ok(())
}
