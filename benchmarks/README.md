# Benchmarks

Keep performance measurements separate from correctness tests. Benchmark
representative circuit sizes and report both core simulation time and end-to-end
Python call time so that PyO3 conversion overhead is visible. Add benchmark
dependencies and executable harnesses only when there is a concrete baseline to
track.

