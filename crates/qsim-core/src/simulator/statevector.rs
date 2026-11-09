use num_complex::Complex64;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    measurement::{basis_bitstring, Counts},
    Circuit, Gate, QSimError,
};

pub struct StateVectorSimulator;

impl StateVectorSimulator {
    pub fn statevector(circuit: &Circuit) -> Result<Vec<Complex64>, QSimError> {
        let qubit_count =
            u32::try_from(circuit.num_qubits()).map_err(|_| QSimError::StateVectorTooLarge)?;
        let dimension = 1usize
            .checked_shl(qubit_count)
            .ok_or(QSimError::StateVectorTooLarge)?;
        let mut state = Vec::new();
        state
            .try_reserve_exact(dimension)
            .map_err(|_| QSimError::StateVectorTooLarge)?;
        state.resize(dimension, Complex64::new(0.0, 0.0));
        state[0] = Complex64::new(1.0, 0.0);

        for gate in circuit.gates() {
            match *gate {
                Gate::H { target } => apply_h(&mut state, target),
                Gate::X { target } => apply_x(&mut state, target),
                Gate::Cx { control, target } => apply_cx(&mut state, control, target),
            }
        }
        Ok(state)
    }
}

pub fn run(circuit: &Circuit, shots: usize, seed: Option<u64>) -> Result<Counts, QSimError> {
    if shots == 0 {
        return Err(QSimError::NoShots);
    }

    let state = StateVectorSimulator::statevector(circuit)?;
    let mut rng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };
    let mut counts = Counts::new();

    for _ in 0..shots {
        let sample = rng.gen::<f64>();
        let mut cumulative_probability = 0.0;
        let mut measured_index = state.len() - 1;
        for (index, amplitude) in state.iter().enumerate() {
            cumulative_probability += amplitude.norm_sqr();
            if sample < cumulative_probability {
                measured_index = index;
                break;
            }
        }
        *counts
            .entry(basis_bitstring(measured_index, circuit.num_qubits()))
            .or_insert(0) += 1;
    }

    Ok(counts)
}

fn apply_h(state: &mut [Complex64], target: usize) {
    let stride = 1usize << target;
    let scale = std::f64::consts::FRAC_1_SQRT_2;
    for base in (0..state.len()).step_by(stride * 2) {
        for offset in 0..stride {
            let zero = state[base + offset];
            let one = state[base + offset + stride];
            state[base + offset] = (zero + one) * scale;
            state[base + offset + stride] = (zero - one) * scale;
        }
    }
}

fn apply_x(state: &mut [Complex64], target: usize) {
    let stride = 1usize << target;
    for base in (0..state.len()).step_by(stride * 2) {
        for offset in 0..stride {
            state.swap(base + offset, base + offset + stride);
        }
    }
}

fn apply_cx(state: &mut [Complex64], control: usize, target: usize) {
    let control_mask = 1usize << control;
    let target_mask = 1usize << target;
    for index in 0..state.len() {
        if index & control_mask != 0 && index & target_mask == 0 {
            state.swap(index, index | target_mask);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bell_circuit_only_measures_correlated_states() {
        let mut circuit = Circuit::new(2).unwrap();
        circuit.h(0).unwrap();
        circuit.cx(0, 1).unwrap();

        let counts = run(&circuit, 1000, Some(7)).unwrap();

        assert_eq!(counts.values().sum::<usize>(), 1000);
        assert!(counts
            .keys()
            .all(|bitstring| bitstring == "00" || bitstring == "11"));
        assert!(counts.contains_key("00"));
        assert!(counts.contains_key("11"));
    }
}
