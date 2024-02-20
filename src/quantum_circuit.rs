use crate::qubit::Qubit;

#[derive(Debug)]
pub enum QuantumGate {
    Hadamard,
    PauliX,
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

    // TODO: Add a second qubit here as an argument for control/target gates (CNOT)
    pub fn apply(&self, qubit: &mut Qubit) {
        for gate in &self.gates {
            match gate {
                Some(QuantumGate::Hadamard) => qubit.hadamard_gate(),
                Some(QuantumGate::PauliX) => qubit.pauli_x_gate(),
                None => {}
            }
        }
    }
}

#[derive(Debug)]
pub struct QuantumCircuit {
    pub qubit_matrix: Vec<QubitLine>,
}

impl QuantumCircuit {
    pub fn new(number_of_qubits: usize) -> Self {
        let mut qubit_matrix = Vec::with_capacity(number_of_qubits);
        for _ in 0..number_of_qubits {
            qubit_matrix.push(QubitLine { gates: vec![None] });
        }

        Self { qubit_matrix }
    }

    pub fn add_gate(&mut self, qubit: usize, gate: QuantumGate) {
        self.qubit_matrix[qubit].add_gate(Some(gate));
    }

    pub fn next_step(&mut self) {
        for qubit_line in &mut self.qubit_matrix {
            qubit_line.add_blank_new_node();
        }
    }
}