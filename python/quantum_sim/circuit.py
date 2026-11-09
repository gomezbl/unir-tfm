from __future__ import annotations

Operation = tuple[str, tuple[int, ...]]


class Circuit:
    """An ordered collection of gates acting on a fixed number of qubits."""

    def __init__(self, num_qubits: int) -> None:
        if isinstance(num_qubits, bool) or not isinstance(num_qubits, int):
            raise TypeError("num_qubits must be an integer")
        if num_qubits < 1:
            raise ValueError("num_qubits must be greater than zero")
        self._num_qubits = num_qubits
        self._operations: list[Operation] = []

    @property
    def num_qubits(self) -> int:
        return self._num_qubits

    def h(self, target: int) -> Circuit:
        return self._append("h", target)

    def x(self, target: int) -> Circuit:
        return self._append("x", target)

    def cx(self, control: int, target: int) -> Circuit:
        self._validate_qubit(control)
        self._validate_qubit(target)
        if control == target:
            raise ValueError("control and target must be different qubits")
        self._operations.append(("cx", (control, target)))
        return self

    def _append(self, name: str, target: int) -> Circuit:
        self._validate_qubit(target)
        self._operations.append((name, (target,)))
        return self

    def _validate_qubit(self, qubit: int) -> None:
        if isinstance(qubit, bool) or not isinstance(qubit, int):
            raise TypeError("qubit indices must be integers")
        if not 0 <= qubit < self._num_qubits:
            raise ValueError(
                f"qubit index {qubit} is out of range for a "
                f"{self._num_qubits}-qubit circuit"
            )

    def _native_operations(self) -> list[Operation]:
        return list(self._operations)
