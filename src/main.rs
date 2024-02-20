mod input;
mod output;
mod quantum_circuit;
mod qubit;

fn main() {
    let inputs = input::get_inputs();

    // Print outputs out
    println!("Inputs:");
    for input in &inputs {
        println!("{}", input);
    }

    // // Example usage
    // let mut circuit = QuantumCircuit::new(3); // Create a circuit with 3 qubits

    // circuit.add_gate(0, QuantumGate::Hadamard); // Add Hadamard gate to qubit 0
    // circuit.add_gate(1, QuantumGate::PauliX); // Add PauliX gate to qubit 1
    // circuit.next_step(4);Q

    // print!("{:?}", circuit);

}