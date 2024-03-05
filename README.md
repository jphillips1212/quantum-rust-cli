# quantum-rust-cli

quantum-rust-cli is a command line interface for creating quantum circuits directly in the terminal and then running them. It can support up to 9 qubits at a time as well as a number of gates for each. 

Currently the only gates supported are Hadamard Gates and Pauli-X Gates but I'm planning to expand it to other Pauli gates aswell as CNOT gates that link qubits together (contributions are welcome!). There is also no current way to measure the qubit at any point without directly interacting with the code, this is the next step. 

# How to run

Clone the repo and from the root directory run `cargo run`.

=======
# quantum-rust-cli

quantum-rust-cli is a command line interface for creating quantum circuits directly in the terminal and then running them. It can support up to 9 qubits at a time as well as a number of gates for each. 

Currently the only gates supported are Hadamard Gates and Pauli-X Gates but I'm planning to expand it to other Pauli gates aswell as CNOT gates that link qubits together (contributions are welcome!). I'm also planning to expand the simulations to running quantum circuits of the same form running multiple times to create statistical averages and properties of each quantum circuit.

# How to run

Clone the repo and from the root directory run `cargo run`

# Initiating your quantum circuit

Once the program is running initiate a new quantum-circuit with the command `QC(4)` where the number is the number of qubits you want your quantum circuit to have. This should print out a new, blank, quantum circuit with the desired number of qubits. e.g:

```

qubit0-----


qubit1-----


qubit2-----


qubit3-----

```

To add gates to the qubits journey use the commands `add(0H)` for a Hadamard gate or `add(0X)` for a Pauli-X gate. Both of these will add the gate to the first step of the 0th qubit. 
Note that any more commands to the 0th qubit will overwrite the current gate that is there. 
For example the commands:
```
QC(4)
add(0H)
add(1X)
add(2H)
add(3X)
```
will generate the circuit:
```

qubit0---H-


qubit1---X-


qubit2---H-


qubit3---X-

```

# Adding more steps to your quantum circuit

Use the `next` command to add another step to each qubit in the circuit. Note that you should build every gate you want at that particular step for each qubit before using the `next` command as there is currently no way to step back and make changes. 
For example the commands:
```
QC(4)
add(0H)
add(1X)
add(2H)
add(3X)
next
add(0X)
add(1H)
add(2X)
add(3H)
```
will generate the circuit:
```

qubit0---H--X-


qubit1---X--H-


qubit2---H--X-


qubit3---X--H-

```
The `next` command can be used indefinitely to build as many steps to the quantum circuit as desired.

# Running the circuit and extracting measurements

Use the `add(0M)` command to add a measurement to the 0th qubit. This will print the state of the qubit as either `|0>` or `|1>`.
This will collapse any wavefunction of the qubit to a measurable state. These measurements can be measured after the circuit is created by using the command `run`
For example the commands:
```
QC(3)
add(0H)
add(1H)
add(2H)
next
add(0M)
add(1M)
add(2M)
```
will generate a circuit of:
```
qubit0---H--M-


qubit1---H--M-


qubit2---H--M-
```
and after using the command `run` will create an output similar to 
```
|0>
|0>
|1>
```
Note that the output will not always be the same due to the random nature of measuring qubits. But over many iterations each qubit should vary between `|0>` and `|1>` at a roughly 50-50 rate.





