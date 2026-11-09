from quantum_sim import Circuit, Simulator


circuit = Circuit(2)
circuit.h(0)
circuit.cx(0, 1)

result = Simulator(seed=7).run(circuit, shots=2000)
print(result.counts)

