extern crate sample;

use super::graph::BufferSet;

pub trait Processor<F: sample::Frame> {
    /// reads input from the first Vector of Frames
    /// and writes it to the second Vector of Frames
    fn process(&mut self, &BufferSet<F>, &mut BufferSet<F>);
    /// return the amount of inputs
    fn inputs_amt(&self) -> usize;
    /// return the amount of outputs
    fn outputs_amt(&self) -> usize;
}
