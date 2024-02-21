#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub alpha: f64,  // Coefficient for |0⟩ state
    pub beta: f64,   // Coefficient for |1⟩ state
    entangled_with: Option<usize>, // Index of the qubit it's entangled with
}

impl Qubit {
    // Create a new qubit in the |0⟩ state
    pub fn new() -> Self {
        Self { alpha: 1.0, beta: 0.0, entangled_with: None }
    }

    // Apply a Hadamard gate to the qubit
    pub fn hadamard_transformation(&mut self) {
        // Hadamard gate matrix
        let h = 1.0 / 2.0f64.sqrt();
        let (alpha, beta) = (self.alpha, self.beta);

        // Update qubit state after applying Hadamard gate
        self.alpha = h * (alpha + beta);
        self.beta = h * (alpha - beta);
    }

    // Apply a pauli_x_gate to the circuit
    pub fn pauli_x_transformation(&mut self) {
        // Pauli-X gate matrix
        let (alpha, beta) = (self.alpha, self.beta);
        self.alpha = beta;
        self.beta = alpha;
    }

    // Measure the qubit, collapsing any superpositional states into either |0⟩ or |1⟩
    pub fn measure(&mut self) {
        // Probability of measuring |0⟩
        let probability_0 = self.alpha.powi(2);
        // Generate a random number between 0 and 1
        let random_number = rand::random::<f64>();
        
        // Update qubit state based on measurement outcome
        if random_number <= probability_0 {
            // Measurement outcome is |0⟩
            self.alpha = 1.0;
            self.beta = 0.0;
        } else {
            // Measurement outcome is |1⟩
            self.alpha = 0.0;
            self.beta = 1.0;
        }

        if self.alpha == 0.0 {
            println!("|0>");
        } else if self.alpha == 1.0 {
            println!("|1>");
        } else {
            println!("Qubit did not collapse to either a |0> or |1> state upon measurement");
        }
    }
    
}