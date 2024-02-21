use crate::quantum_circuit::{QuantumCircuit, QuantumGate};

pub fn print_quantum_circuit(quantum_circuit: &QuantumCircuit) {

    println!("_______________Build your quantum circuit_______________");
    println!("____Input NH to add a hadamard gate to the Nth Qubit____");
    println!("_____Input NX to add a PauliX gate to the Nth Qubit_____");
    println!("Input next to go to the next node in the quantum circuit");
    println!();
    println!();
    println!();
    // Loop through each qubit
    for (index, qubit_line) in quantum_circuit.qubit_matrix.iter().enumerate() {
        let (mut top_line_print, mut middle_line_print, mut bottom_line_print) = (String::new(), String::new(), String::new());

        // Loop through each gate of each qubit
        for gate in &qubit_line.gates {
            match gate {
                None => {
                    top_line_print += "   ";
                    middle_line_print += "---";
                    bottom_line_print += "   ";
                }
                Some(QuantumGate::Hadamard) => {
                    top_line_print += "   ";
                    middle_line_print += "-H-";
                    bottom_line_print += "   ";
                }
                Some(QuantumGate::PauliX) => {
                    top_line_print += "   ";
                    middle_line_print += "-X-";
                    bottom_line_print += "   ";
                }
                Some(QuantumGate::Measurement) => {
                    top_line_print += "   ";
                    middle_line_print += "-M-";
                    bottom_line_print += "   ";
                }
            }
        }

        println!("        {}", top_line_print);
        println!("qubit{}--{}", index, middle_line_print);
        println!("        {}", top_line_print);

    }
}
