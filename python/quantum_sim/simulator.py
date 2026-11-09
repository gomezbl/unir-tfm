from __future__ import annotations

from typing import Optional

from . import _core
from .circuit import Circuit
from .result import Result


class Simulator:
    """Run circuits using the native Rust state-vector engine."""

    def __init__(self, seed: Optional[int] = None) -> None:
        if seed is not None and (isinstance(seed, bool) or not isinstance(seed, int)):
            raise TypeError("seed must be an integer or None")
        if seed is not None and not 0 <= seed < 2**64:
            raise ValueError("seed must fit in an unsigned 64-bit integer")
        self._seed = seed

    def run(self, circuit: Circuit, shots: int = 1024) -> Result:
        if not isinstance(circuit, Circuit):
            raise TypeError("circuit must be a Circuit")
        if isinstance(shots, bool) or not isinstance(shots, int):
            raise TypeError("shots must be an integer")
        if shots < 1:
            raise ValueError("shots must be greater than zero")
        counts = _core.run(
            circuit.num_qubits,
            circuit._native_operations(),
            shots,
            self._seed,
        )
        return Result(counts, shots)
