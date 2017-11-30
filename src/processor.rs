extern crate sample;

use super::graph::BufferSet;
use super::graph::FrameSet;

pub trait Processor<F: sample::Frame> {
    /// read input from the first Vector of Frames
    /// and write it to the second Vector of Frames
    fn process(&mut self, 
               inputs: &BufferSet<F>,
               outputs: &mut BufferSet<F>,
               input_frameset: &mut FrameSet<F>,
               output_frameset: &mut FrameSet<F>) {
        // return because no samples to process
        if inputs.len() == 0 {
            return;
        }

        // return because first FrameSet has no Frames
        if inputs[0].len() == 0 {
            return;
        }

        for sample in 0..inputs[0].len() {
            
        }
    }

    fn frame_process(&mut self, &FrameSet<F>, &mut FrameSet<F>) {}

    /// return the amount of inputs
    fn inputs_amt(&self) -> usize;

    /// return the amount of outputs
    fn outputs_amt(&self) -> usize;
}
