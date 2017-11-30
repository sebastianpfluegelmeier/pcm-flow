extern crate sample;

use super::graph::BufferSet;
use super::graph::FrameSet;

/// The trait every signal processor has to implement.
/// The inputs_amt function should return the number of inputs
/// and the output_amt function should return the number of outputs of the processor 
///
/// Either the process or the frame_process method has to be overriden.
/// If none of them are overriden the signal processor does nothing.
pub trait Processor<F: sample::Frame> {
    /// Override this function if you want to work on BufferSets.
    /// Read input from the input BufferSet
    /// and write it to the output BufferSet
    fn process(&mut self, inputs: &BufferSet<F>, outputs: &mut BufferSet<F>) {
        for i in 0..inputs.len() {
            self.frame_process(&inputs[i], &mut outputs[i]);
        }
    }

    /// Override this function if you want to work on FrameSets
    /// Read input from the input FrameSet
    /// and write it to the output FrameSet
    fn frame_process(&mut self, &FrameSet<F>, &mut FrameSet<F>) {}

    /// return the amount of inputs
    fn inputs_amt(&self) -> usize;

    /// return the amount of outputs
    fn outputs_amt(&self) -> usize;
}
