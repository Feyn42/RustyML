
//
use crate::utils::dot;

// Single Perceptron Model
pub struct Perceptron {

    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,

}

// `Perceptron` Implementations
impl Perceptron {

    // Model initialization
    pub fn init(input_size: usize, learning_rate: f64) -> Self {

        // Create weights & bias
        let weights = vec![0.0; input_size];
        let bias = 0.0;

        // Setup `self`
        Self { weights, bias, learning_rate }

    }

    // Model training
    pub fn train(&mut self, input: &[f64], target: f64) {

        // Calculate error
        let pred = self.predict(input);
        let error = target - pred;

        // Update weights & bias
        for i in 0..self.weights.len() {
            self.weights[i] += self.learning_rate * error * input[i];
        }
        self.bias += self.learning_rate * error;

    }

    // Model prediction
    pub fn predict(&self, input: &[f64]) -> f64 {

        // Calculate weighted sum
        let weighted_sum = dot(&input, &self.weights);

        // Activation function: unit step function
        if weighted_sum > 0.0 { 1.0 } else { 0.0 }

    }

}