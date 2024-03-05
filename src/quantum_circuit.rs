use std::usize;

use crate::qubit::Qubit;

// TODO Create a way of recording measurements when their taken along with their position within the qubit matrix
pub static mut MEASUREMENT_MATRIX: Option<Vec<Vec<Option<String>>>> = None;

#[derive(Debug)]
pub enum QuantumGate {
    Hadamard,
    PauliX,
    Measurement,
}

impl QuantumGate {
    pub fn apply(&self, qubit: &mut Qubit, qubit_line: &QubitLine, node_index: usize) {
        match self {
            QuantumGate::Hadamard => qubit.hadamard_transformation(),
            QuantumGate::PauliX => qubit.pauli_x_transformation(),
            QuantumGate::Measurement => qubit.measure(),
        }
    }
}

#[derive(Debug)]
pub struct QubitLine {
    pub gates: Vec<Option<QuantumGate>>,
}

impl QubitLine {
    pub fn add_gate(&mut self, gate: Option<QuantumGate>) {
        let column_len = self.gates.len();
        self.gates[column_len - 1] = gate;
    }

    pub fn add_blank_new_node(&mut self) {
        self.gates.push(None);
    }
}

#[derive(Debug)]
pub struct QuantumCircuit {
    pub qubit_matrix: Vec<QubitLine>,
    pub qubits: Vec<Qubit>,
}

impl QuantumCircuit {
    pub fn new(number_of_qubits: usize) -> Self {
        let mut qubit_matrix = Vec::with_capacity(number_of_qubits);
        let mut qubits = Vec::with_capacity(number_of_qubits);
        for _ in 0..number_of_qubits {
            qubit_matrix.push(QubitLine { gates: vec![None] });
            let new_qubit = Qubit::new();
            qubits.push(new_qubit);
        }

        Self { qubit_matrix, qubits }
    }

    pub fn add_gate(&mut self, qubit: usize, gate: QuantumGate) {
        self.qubit_matrix[qubit].add_gate(Some(gate));
    }

    pub fn next_step(&mut self) {
        for qubit_line in &mut self.qubit_matrix {
            qubit_line.add_blank_new_node();
        }
    }

    pub fn run_circuit(&mut self) {
        // Get the length of the quantum circuit from the first qubit
        let num_columns = self.qubit_matrix.first().map_or(0, |line| line.gates.len());

        // Loop through each "node" of the quantum circuit one-by-one
        for column_index in 0..num_columns {
            // Loop through each qubit of the node one-by-one (async possible here?)
            for (qubit_index, qubit_line) in self.qubit_matrix.iter_mut().enumerate() {
                // Get the gate option of each node and unwrap it. Apply the gate if it's not None to the correct qubit
                if let Some(gate_option) = qubit_line.gates.get(column_index) {
                    if let Some(gate) = gate_option {
                        gate.apply(&mut self.qubits[qubit_index], qubit_line, column_index);
                    }
                }
            }
        }
    }

    pub fn run_circuit_with_debug(&mut self) {
        // Get the length of the quantum circuit from the first qubit
        let num_columns = self.qubit_matrix.first().map_or(0, |line| line.gates.len());

        // Loop through each "node" of the quantum circuit one-by-one
        for column_index in 0..num_columns {
            // Loop through each qubit of the node one-by-one (async possible here?)
            for (qubit_index, qubit_line) in self.qubit_matrix.iter_mut().enumerate() {
                // Get the gate option of each node and unwrap it. Apply the gate if it's not None to the correct qubit
                if let Some(gate_option) = qubit_line.gates.get(column_index) {
                    if let Some(gate) = gate_option {
                        gate.apply(&mut self.qubits[qubit_index], &qubit_line, column_index);
                        println!("Gate {:?} applied to qubit {:?}", gate, qubit_index);
                    }
                }
            }
        }        
    }
}