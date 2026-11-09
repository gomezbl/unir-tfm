use std::collections::HashMap;

use pyo3::prelude::*;
use qsim_core::{run as run_simulation, Circuit, Gate};

use crate::errors::to_python_error;

#[pyfunction]
#[pyo3(signature = (num_qubits, operations, shots, seed=None))]
pub(crate) fn run(
    num_qubits: usize,
    operations: Vec<(String, Vec<usize>)>,
    shots: usize,
    seed: Option<u64>,
) -> PyResult<HashMap<String, usize>> {
    let mut circuit = Circuit::new(num_qubits).map_err(to_python_error)?;
    for (name, qubits) in operations {
        let gate = match (name.as_str(), qubits.as_slice()) {
            ("h", [target]) => Gate::H { target: *target },
            ("x", [target]) => Gate::X { target: *target },
            ("cx", [control, target]) => Gate::Cx {
                control: *control,
                target: *target,
            },
            ("h" | "x" | "cx", _) => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid qubit arguments for gate '{name}'"
                )));
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "unsupported gate '{name}'"
                )));
            }
        };
        circuit.add_gate(gate).map_err(to_python_error)?;
    }
    run_simulation(&circuit, shots, seed).map_err(to_python_error)
}
