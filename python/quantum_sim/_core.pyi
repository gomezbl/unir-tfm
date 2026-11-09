from typing import Optional

def run(
    num_qubits: int,
    operations: list[tuple[str, tuple[int, ...]]],
    shots: int,
    seed: Optional[int] = ...,
) -> dict[str, int]: ...
