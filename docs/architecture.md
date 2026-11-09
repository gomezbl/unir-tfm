# Architecture

## Dependency direction

`qsim-core` owns the domain model and simulation algorithms and has no Python
dependency. `qsim-python` translates Python inputs to core types and maps domain
errors to Python `ValueError`s. `quantum_sim` is the supported public API; users
should not need to depend on the private `_core` extension directly.

```text
Python callers
    -> quantum_sim (Circuit, Simulator, Result)
        -> quantum_sim._core (PyO3)
            -> qsim-core (circuit, gates, statevector, measurement)
```

## Current simulation contract

- State is stored as a dense vector of complex amplitudes in little-endian
  qubit indexing: qubit `q` is bit `q` of the basis-state index.
- Displayed measurement strings reverse that index order, showing
  `q[n-1] ... q[0]`.
- Gates supported initially are Hadamard, Pauli-X, and controlled-X.
- `run` samples the final state in the computational basis. It does not expose
  mid-circuit measurement or collapse/reuse of a state.
- An explicit seed makes sampling reproducible for a given implementation and
  dependency version. With no seed, entropy seeds the Rust RNG.

## Extension points

Add new gate definitions and state-vector kernels in `qsim-core`; keep
input-facing validation in both the Python API and core boundary. Alternative
simulation strategies can be added below `simulator/` behind a common simulator
interface without making the PyO3 layer own quantum logic. Keep crossings coarse
grained: submit the full operation sequence per run rather than crossing into
Rust for each gate.

