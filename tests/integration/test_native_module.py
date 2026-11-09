import pytest

from quantum_sim import _core


def test_native_module_translates_invalid_gate_arguments() -> None:
    with pytest.raises(ValueError, match="invalid qubit arguments"):
        _core.run(1, [("h", ())], 1)


def test_native_module_rejects_unsupported_gate() -> None:
    with pytest.raises(ValueError, match="unsupported gate"):
        _core.run(1, [("t", (0,))], 1)

