# pcm-flow

A library for building big synthesizers and effects from small modules.

## Usage

Add pcm-flow to your Cargo.toml
```
[dependencies]
pcm-flow = "0.1.0"
```

``` rust
extern crate pcm_flow;

use pcm_flow::graph::Graph;
use pcm_flow::processor::Processor;

fn main() {
    let mut graph = Graph::new();
    let mixer = graph.add_processor(Box::new(Mixer{}));
    let distortion = graph.add_processor(Box::new(Distortion{}));
    let delay = graph.add_processor(Box::new(Delay{
                                                ringbuffer: vec![vec![0.0; 2]; 1000],
                                                index: 0}));
    graph.add_connection(&(distortion, 0), &(mixer, 0)).unwrap();
    graph.add_connection(&(delay, 0),      &(mixer, 1)).unwrap();
    graph.set_input_amt(1);
    graph.set_output_amt(1);
    graph.connect_input(0, (distortion, 0)).unwrap();
    graph.connect_input(0, (delay, 0)).unwrap();
    graph.connect_output(0, (mixer, 0)).unwrap();
    let mut input = vec![[3.1 ,3.1]];
    let mut output = vec![[0.0, 0.0]];
    graph.process(&mut input, &mut output)
}

struct Mixer {}

impl Processor<[f32; 2]> for Mixer {
    fn process(&mut self, inputs: &mut Vec<[f32; 2]>, outputs: &mut Vec<[f32; 2]>) {
        for i in 0..2 {
            outputs[0][i] = inputs[0][i] + inputs[1][i];
        }
    }
    fn inputs_amt(&self) -> usize { 2 }
    fn outputs_amt(&self) -> usize { 1 }
}

struct Distortion {}

impl Processor<[f32; 2]> for Distortion {
    fn process(&mut self, inputs: &mut Vec<[f32; 2]>, outputs: &mut Vec<[f32; 2]>) {
        for i in 0..2 {
            outputs[0][i] = inputs[0][i].max(0.5).min(-0.5);
        }
    }
    fn inputs_amt(&self) -> usize { 2 }
    fn outputs_amt(&self) -> usize { 1 }
}

struct Delay {
    ringbuffer: Vec<Vec<f32>>,
    index: usize,
}

impl Processor<[f32; 2]> for Delay {
    fn process(&mut self, inputs: &mut Vec<[f32; 2]>, outputs: &mut Vec<[f32; 2]>) {
        for i in 0..2 {
            outputs[0][i] = self.ringbuffer[i][self.index];
            self.ringbuffer[i][self.index] = inputs[0][i];
            self.index += 1;
            if self.index >= self.ringbuffer.len() {
                self.index = 0;
            }
        }
    }
    fn inputs_amt(&self) -> usize { 2 }
    fn outputs_amt(&self) -> usize { 1 }
}
```
