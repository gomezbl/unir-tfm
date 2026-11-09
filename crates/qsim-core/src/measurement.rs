use std::collections::HashMap;

pub type Counts = HashMap<String, usize>;

pub(crate) fn basis_bitstring(index: usize, num_qubits: usize) -> String {
    (0..num_qubits)
        .rev()
        .map(|qubit| if index & (1 << qubit) == 0 { '0' } else { '1' })
        .collect()
}
