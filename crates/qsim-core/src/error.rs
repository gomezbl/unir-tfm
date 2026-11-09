use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QSimError {
    #[error("a circuit must contain at least one qubit")]
    NoQubits,
    #[error("qubit index {index} is out of range for a {num_qubits}-qubit circuit")]
    QubitOutOfRange { index: usize, num_qubits: usize },
    #[error("a controlled gate must use distinct control and target qubits")]
    DuplicateControlAndTarget,
    #[error("the requested state vector is too large to allocate")]
    StateVectorTooLarge,
    #[error("shots must be greater than zero")]
    NoShots,
}
