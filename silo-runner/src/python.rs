use pyo3::exceptions::PyRuntimeError;
use pyo3::types::PyDict;
use pyo3::Python;

/// Python executor.
pub struct PythonExecutor {}

impl PythonExecutor {
    /// Creates and returns a new PythonExecutor.
    pub fn new() -> Self {
        Self {}
    }

    /// Executes code synchronously.
    pub fn execute_sync(&self, raw_code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let locals = PyDict::new(py);
        py.run(raw_code, None, Some(locals))
            .expect("Error running python");

        let ret = locals.get_item("ret").ok_or(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to get variable",
        )))?;

        let output: String = ret.repr().unwrap().extract().unwrap();
        Ok(output)
    }
}
