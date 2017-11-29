# pcm-flow

A library for building big synthesizers and effects from small modules.
You can write structs implementing the Processor Trait and chain them together in a flexible way.
This library is still in early development and it is not advised to use it yet

## Usage

Add pcm-flow to your Cargo.toml
```
[dependencies]
pcm-flow = "0.3.0"
```
# A simple Program using pcm-flow
This program shows how to use pcm-flow in a very useless but simple way.
We define a struct implementing the Processor trait which just takes an input and passes it to its output.
Then we make two instances of this struct and chain them together.

``` rust
extern crate pcm_flow;

use pcm_flow::graph::Graph;
use pcm_flow::processor::Processor;

fn main() {
    // create a new graph Struct, it is the main container for our Processors
    let mut graph = Graph::new(1);
    // Add two PassThrough structs to the graph and store their IDs in variables
    let pass_through1 = graph.add_processor(Box::new(PassThrough{}));
    let pass_through2 = graph.add_processor(Box::new(PassThrough{}));
    // connect the two processors
    graph.add_connection(&(pass_through1, 0), &(pass_through2, 0)).unwrap();
    // add an input to the graph
    graph.set_input_amt(1);
    // add an output to the graph
    graph.set_output_amt(1);
    // connect the input to the first Processor
    graph.connect_input(0, (pass_through1, 0)).unwrap();
    // connect the second Processor to the Output
    graph.connect_output(0, (pass_through2, 0)).unwrap();
    graph.process
}

// The struct we define here, takes one input and passes the signal to the output
struct PassThrough {}

impl Processor<[f32; 2]> for PassThrough {
    fn process(&mut self, inputs: &Vec<Vec<[f32; 2]>>, outputs: &mut Vec<Vec<[f32; 2]>>) {
        for i in 0..2 {
            for sample in 0..1 {
                outputs[0][sample][i] = inputs[0][sample][i];
            }
        }
    }
    fn inputs_amt(&self) -> usize { 1 }
    fn outputs_amt(&self) -> usize { 1 }
}
```
