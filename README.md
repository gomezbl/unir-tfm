# quantum-sim

A hybrid quantum-circuit simulator with a Rust state-vector engine and a Python
API. Rust owns the simulation and sampling; PyO3 exposes a small native boundary,
and Python provides the user-facing circuit and result types.

## Requirements

- Rust toolchain (Cargo and rustc)
- Python 3.9+
- Maturin 1.x

## Build and test

From the project root:

```console
python -m pip install ".[test]"
maturin develop
cargo test --workspace
python -m pytest
```

`maturin develop` builds and installs the native extension into the active
Python environment. For a distributable wheel, run `maturin build --release`.

## Quick start

```python
from quantum_sim import Circuit, Simulator

circuit = Circuit(2)
circuit.h(0)
circuit.cx(0, 1)

result = Simulator(seed=42).run(circuit, shots=1000)
print(result.counts)
```

The initial engine supports Hadamard (`h`), Pauli-X (`x`), and controlled-X
(`cx`) gates, followed by computational-basis measurement. Bitstrings are
displayed in descending qubit order (`q[n-1] ... q[0]`); consequently the
two-qubit Bell circuit returns `00` and `11`.

## Project map

- `crates/qsim-core`: Python-independent circuit model, state-vector engine,
  measurement, and domain errors.
- `crates/qsim-python`: PyO3 extension module (`quantum_sim._core`).
- `python/quantum_sim`: public Python API and native-module type stub.
- `tests`: Rust unit tests and Python API/integration tests.
- `examples`: runnable usage examples.
- `docs`: architecture and behavior notes.
- `benchmarks`: notes and entry points for performance measurements.

See [docs/architecture.md](docs/architecture.md) for module boundaries and
extension points.
