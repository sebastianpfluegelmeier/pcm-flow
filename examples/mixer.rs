extern crate pcm_flow;

use pcm_flow::graph::Graph;
use pcm_flow::processor::Processor;

fn main() {
    // create a new graph Struct, it is the main container for our Processors
    let mut graph = Graph::new(1);
    let mixer = graph.add_processor(Box::new(Mixer {}));
    let distortion = graph.add_processor(Box::new(Distortion {}));
    let delay = graph.add_processor(Box::new(Delay {
        ringbuffer: vec![vec![0.0; 2]; 10],
        index: 0,
    }));
    graph.add_connection(&(distortion, 0), &(mixer, 0)).unwrap();
    graph.add_connection(&(delay, 0), &(mixer, 1)).unwrap();
    graph.set_input_amt(1);
    graph.set_output_amt(1);
    graph.connect_input(0, (distortion, 0)).unwrap();
    graph.connect_input(0, (delay, 0)).unwrap();
    graph.connect_output(0, (mixer, 0)).unwrap();
    let input = vec![vec![[3.1, 3.1]]];
    let mut output = vec![vec![[0.0, 0.0]]];
    graph.process(&input, &mut output);
    assert_eq!(output[0][0][0], 0.5);
    for _ in 0..8 {
        graph.process(&input, &mut output);
    }
    assert_eq!(output[0][0][0], 0.5 + 3.1);
}

// The Mixer struct we define here, takes two inputs and outputs the sum
struct Mixer {}

impl Processor<[f32; 2]> for Mixer {
    fn process(&mut self, inputs: &Vec<Vec<[f32; 2]>>, outputs: &mut Vec<Vec<[f32; 2]>>) {
        for channel in 0..2 {
            for sample in 0..inputs.len() {
                outputs[sample][0][channel] = inputs[sample][0][channel] + inputs[sample][1][channel];
            }
        }
    }
    fn inputs_amt(&self) -> usize {
        2
    }
    fn outputs_amt(&self) -> usize {
        1
    }
}

// The Distortion struct defined here takes a input and clips the signal at 0.5 and -0.5
struct Distortion {}

impl Processor<[f32; 2]> for Distortion {
    fn process(&mut self, inputs: &Vec<Vec<[f32; 2]>>, outputs: &mut Vec<Vec<[f32; 2]>>) {
        for channel in 0..2 {
            for sample in 0..inputs.len() {
                outputs[sample][0][channel] = inputs[sample][0][channel].min(0.5).max(-0.5);
            }
        }
    }
    fn inputs_amt(&self) -> usize {
        1
    }
    fn outputs_amt(&self) -> usize {
        1
    }
}

// The Delay struct takes a input and delays its output by a fixed amounth of samples
struct Delay {
    ringbuffer: Vec<Vec<f32>>,
    index: usize,
}

impl Processor<[f32; 2]> for Delay {
    fn process(&mut self, inputs: &Vec<Vec<[f32; 2]>>, outputs: &mut Vec<Vec<[f32; 2]>>) {
        for channel in 0..2 {
            for sample in 0..inputs.len() {
                outputs[sample][0][channel] = self.ringbuffer[self.index][channel];
                self.ringbuffer[self.index][channel] = inputs[sample][0][channel];
                self.index += 1;
                if self.index >= self.ringbuffer.len() {
                    self.index = 0;
                }
            }
        }
    }
    fn inputs_amt(&self) -> usize {
        1
    }
    fn outputs_amt(&self) -> usize {
        1
    }
}
