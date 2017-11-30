extern crate sample;

use super::graph::BufferSet;
use super::graph::FrameSet;

pub trait Processor<F: sample::Frame> {
    /// read input from the first Vector of Frames
    /// and write it to the second Vector of Frames
    fn process(&mut self, inputs: &BufferSet<F>, outputs: &mut BufferSet<F>);

    fn frame_process(&mut self, &FrameSet<F>, &mut FrameSet<F>) {}

    /// return the amount of inputs
    fn inputs_amt(&self) -> usize;

    /// return the amount of outputs
    fn outputs_amt(&self) -> usize;
}
