use crate::{Gate, QSimError};

#[derive(Clone, Debug)]
pub struct Circuit {
    num_qubits: usize,
    gates: Vec<Gate>,
}

impl Circuit {
    pub fn new(num_qubits: usize) -> Result<Self, QSimError> {
        if num_qubits == 0 {
            return Err(QSimError::NoQubits);
        }
        Ok(Self {
            num_qubits,
            gates: Vec::new(),
        })
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn gates(&self) -> &[Gate] {
        &self.gates
    }

    pub fn add_gate(&mut self, gate: Gate) -> Result<(), QSimError> {
        for index in gate.qubits().into_iter().flatten() {
            if index >= self.num_qubits {
                return Err(QSimError::QubitOutOfRange {
                    index,
                    num_qubits: self.num_qubits,
                });
            }
        }
        if let Gate::Cx { control, target } = gate {
            if control == target {
                return Err(QSimError::DuplicateControlAndTarget);
            }
        }
        self.gates.push(gate);
        Ok(())
    }

    pub fn h(&mut self, target: usize) -> Result<(), QSimError> {
        self.add_gate(Gate::H { target })
    }

    pub fn x(&mut self, target: usize) -> Result<(), QSimError> {
        self.add_gate(Gate::X { target })
    }

    pub fn cx(&mut self, control: usize, target: usize) -> Result<(), QSimError> {
        self.add_gate(Gate::Cx { control, target })
    }
}
