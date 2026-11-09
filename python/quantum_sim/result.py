from __future__ import annotations

from collections.abc import Mapping


class Result:
    """Counts from computational-basis measurements."""

    def __init__(self, counts: Mapping[str, int], shots: int) -> None:
        self._counts = dict(counts)
        self._shots = shots

    @property
    def counts(self) -> dict[str, int]:
        return self._counts.copy()

    @property
    def shots(self) -> int:
        return self._shots

    def probabilities(self) -> dict[str, float]:
        return {bitstring: count / self._shots for bitstring, count in self._counts.items()}

    def __repr__(self) -> str:
        return f"Result(shots={self._shots}, counts={self._counts!r})"

