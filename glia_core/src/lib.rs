pub mod core;

#[cfg(feature = "python")]
pub mod python_module;

#[cfg(feature = "r")]
pub mod r_module;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
#[pyo3(name = "glia_core")]
fn glia_core_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<python_module::PyPushResult>()?;
    m.add_function(wrap_pyfunction!(python_module::push_telemetry, m)?)?;
    Ok(())
}

#[cfg(feature = "r")]
use extendr_api::prelude::*;

#[cfg(feature = "r")]
extendr_module! {
    mod gliar;
    fn r_module::push_telemetry;
}