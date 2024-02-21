use std::io::{self, Write};

use crate::quantum_circuit::{QuantumCircuit, QuantumGate};
use crate::output::print_quantum_circuit;

pub fn get_inputs() -> Vec<String> {
    let mut inputs = Vec::new();
    let mut quantum_circuit_option: Option<QuantumCircuit> = None;

    // Prompt the user for input
    println!("_______________Build your quantum circuit_______________");
    println!("__Input QC(N) where N is the number of qubits you want__");

    loop {
        io::stdout().flush().unwrap();

        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();

        // Check for quantum circuit input
        match input.as_str() {
            _ if input.starts_with("QC(") && input.ends_with(')') => {
                if quantum_circuit_option.is_some() {
                    println!("Error: Quantum Circuit has already been declared with a number of qubits. Start again to create a circuit with a different number of qubits")
                } else {
                    let qubit_amount = input.as_str().trim_start_matches("QC(").trim_end_matches(")");
                    if let Ok(number_of_qubits) = qubit_amount.parse::<usize>() {
                        let circuit = QuantumCircuit::new(number_of_qubits);
                        print_quantum_circuit(&circuit);
                        quantum_circuit_option = Some(circuit);
                    } else {
                        println!("Error: Number of qubits needs to be initiated as an integer");
                    }
                } 
            } 
            _ if input.eq("next") => {
                match quantum_circuit_option {
                    None => {
                        println!{"The quantum circuit has not been initialised with a number of qubits. Use QC(N) to create a quantum circuit with N qubits where N is an integer"};
                    }
                    Some(ref mut quantum_circuit) => {
                        quantum_circuit.next_step();
                        print_quantum_circuit(&quantum_circuit);
                    }
                } 
            }
            _ if input.starts_with("add(") && input.ends_with(")") => {
                match quantum_circuit_option {
                    None => {
                        println!{"The quantum circuit has not been initialised with a number of qubits. Use QC(N) to create a quantum circuit with N qubits where N is an integer"};
                    }
                    Some(ref mut quantum_circuit) => {
                        let gate_to_be_added = input.as_str().trim_start_matches("add(").trim_end_matches(")");
                        add_gate(quantum_circuit, gate_to_be_added.to_string());
                        print_quantum_circuit(&quantum_circuit);
                    }
                }
            }
            "run" => {
                match quantum_circuit_option {
                    None => {
                        println!{"The quantum circuit has not been initialised with a number of qubits. Use QC(N) to create a quantum circuit with N qubits where N is an integer"};
                    }
                    Some(ref mut quantum_circuit) => {
                        quantum_circuit.run_circuit();
                    }
                } 
            }
            "debug" => {
                match quantum_circuit_option {
                    None => {
                        println!{"The quantum circuit has not been initialised with a number of qubits. Use QC(N) to create a quantum circuit with N qubits where N is an integer"};
                    }
                    Some(ref mut quantum_circuit) => {
                        quantum_circuit.run_circuit_with_debug();
                    }
                } 
            }
            "end" => break,
            _ => println!("Error: Invalid input privided"),
        }

        // Add input to the vector
        inputs.push(input);
    }

    inputs
}

fn add_gate(quantum_circuit: &mut QuantumCircuit, gate_to_be_added: String) -> &mut QuantumCircuit {
    let mut qubit_position_end: Option<usize> = None; 

    // Find the index where the declaration of the qubit position(s) end
    for (index, ch) in gate_to_be_added.chars().enumerate() {
        if !ch.is_numeric() {
            qubit_position_end = Some(index);
            break;
        }
    }

    if let Some(qubit_position_end_index) = qubit_position_end {
        if qubit_position_end_index == 0 || qubit_position_end_index >= 3 {
            println!("Adding a new gate can only have 1 or 2 qubit positions provided depending on the gate. For example: 1H or 23CNOT");
            return quantum_circuit;
        } 

        // Split the string based on the index of where the qubit position index ends
        let (qubit_positions_str, new_gate_str) = gate_to_be_added.split_at(qubit_position_end_index);

        // Parse qubit_positions as usize
        let qubit_positions = match qubit_positions_str.parse::<usize>() {
            Ok(pos) => pos,
            Err(_) => {
                println!("Invalid qubit position(s) provided");
                return quantum_circuit;
            }
        };

        // Match new_gate_str to QuantumGate enum variant
        let new_gate = match new_gate_str {
            "H" => QuantumGate::Hadamard,
            "X" => QuantumGate::PauliX,
            "M" => QuantumGate::Measurement,
            _ => {
                println!("Unknown gate: {}", new_gate_str);
                return quantum_circuit;
            }
        };

        quantum_circuit.add_gate(qubit_positions, new_gate);
        return quantum_circuit;
    } else {
        println!("The quantum gate to be added needs to start with a qubit position or positions. For example: 1H or 23CNOT");
        return quantum_circuit;
    }

} 