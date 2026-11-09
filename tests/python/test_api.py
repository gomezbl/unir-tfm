import pytest

from quantum_sim import Circuit, Simulator


def test_bell_circuit_returns_correlated_measurements() -> None:
    circuit = Circuit(2).h(0).cx(0, 1)

    result = Simulator(seed=42).run(circuit, shots=500)

    assert result.shots == 500
    assert sum(result.counts.values()) == 500
    assert set(result.counts) <= {"00", "11"}
    assert set(result.counts) == {"00", "11"}
    assert sum(result.probabilities().values()) == pytest.approx(1.0)


def test_circuit_rejects_invalid_qubit_indices() -> None:
    with pytest.raises(ValueError, match="out of range"):
        Circuit(2).x(2)


def test_circuit_rejects_same_control_and_target() -> None:
    with pytest.raises(ValueError, match="different qubits"):
        Circuit(2).cx(1, 1)


def test_run_requires_positive_shot_count() -> None:
    with pytest.raises(ValueError, match="greater than zero"):
        Simulator().run(Circuit(1), shots=0)

