extern crate petgraph;
extern crate sample;

use processor::Processor;
use self::sample::{Frame, Sample};
use self::petgraph::graph::Graph as PetGraph;
use std::collections::HashMap;
use std::collections::HashSet;

pub type PortId = (usize, usize);
pub type Buffer<F> = Vec<F>;
pub type FrameSet<F> = Vec<F>;
pub type BufferSet<F> = Vec<FrameSet<F>>;

/// The main container struct for Processors.
/// Processors can be added and connected in arbitrary
/// ways as long there are no cyclic connections.
/// All Processors must be from the same Frame type.
/// A graph has an arbitrary number of inputs and outputs
/// which can be connected to processors.
/// These inputs and outputs are called graph inputs and graph outputs.
pub struct Graph<F> {
    // contains all processors
    processors: Vec<Box<Processor<F>>>,
    // buffers that contains the graph inputs
    graph_input_buffers: BufferSet<F>,
    // buffers that contain the graph outputs
    graph_output_buffers: BufferSet<F>,
    // input buffer sets for all processors
    input_buffers: Vec<BufferSet<F>>,
    // output buffer sets for all processors
    output_buffers: Vec<BufferSet<F>>,
    // a hash map describing all connections from port to port
    connections: HashMap<PortId, HashSet<PortId>>,
    // a list of connections from the inputs to nodes
    input_connections: HashMap<usize, HashSet<PortId>>,
    // a list of connections from nodes to the outputs
    output_connections: HashMap<usize, HashSet<PortId>>,
    // stores all processor indexes sorted topologically
    topological_sorting: Vec<usize>,
    // amount of Frames processed for one process()
    buffersize: usize,
}

impl<F> Graph<F>
where
    F: Frame,
{
    /// Create a new empty Graph
    pub fn new(buffersize: usize) -> Self {
        Graph {
            processors: Vec::new(),
            graph_input_buffers: vec![],
            graph_output_buffers: vec![],
            input_connections: HashMap::new(),
            output_connections: HashMap::new(),
            connections: HashMap::new(),
            topological_sorting: Vec::new(),
            input_buffers: Vec::new(),
            output_buffers: Vec::new(),
            buffersize: buffersize,
        }
    }

    /// Add a new processor to the Graph. Its ID gets returned.
    pub fn add_processor(&mut self, processor: Box<Processor<F>>) -> usize {
        let index = self.processors.len();
        self.input_buffers
            .push(empty_buffer(processor.inputs_amt(), self.buffersize));
        self.output_buffers
            .push(empty_buffer(processor.outputs_amt(), self.buffersize));
        for i in 0..processor.outputs_amt() {
            self.connections.insert((index, i), HashSet::new());
        }
        self.processors.push(processor);
        return index;
    }

    /// Connect an input to a processor
    pub fn connect_input(&mut self, input: usize, port: PortId) -> Result<(), String> {
        if !self.inport_exists(port) {
            return Err(format!("port {} does not exist on node {}", port.0, port.1));
        }
        match self.input_connections.get_mut(&input) {
            Some(x) => {
                x.insert(port);
                Ok(())
            }
            None => Err(format!("input {} does not exist", input)),
        }
    }

    /// connect an output to a processor
    pub fn connect_output(&mut self, output: usize, port: PortId) -> Result<(), String> {
        if !self.outport_exists(port) {
            return Err(format!("port {} does not exist on node {}", port.0, port.1));
        }
        match self.output_connections.get_mut(&output) {
            Some(x) => {
                x.insert(port);
                Ok(())
            }
            None => Err(format!("input {} does not exist", output)),
        }
    }

    /// set the amount of inputs
    pub fn set_input_amt(&mut self, inputs: usize) {
        self.graph_input_buffers = empty_buffer(inputs, self.buffersize);
        self.input_connections = HashMap::new();
        for i in 0..inputs {
            self.input_connections.insert(i, HashSet::new());
        }
    }

    /// set the amount of outputs
    pub fn set_output_amt(&mut self, outputs: usize) {
        self.graph_output_buffers = empty_buffer(outputs, self.buffersize);
        self.output_connections = HashMap::new();
        for i in 0..outputs {
            self.output_connections.insert(i, HashSet::new());
        }
    }

    /// add aconnection between two ports
    /// either returns an Ok(connection Id) or in case of a cycle or an invalid
    /// port, a Err(Description)
    pub fn add_connection(&mut self, &source_id: &PortId, &dest_id: &PortId) -> Result<(), String> {
        // check if src port exists
        match self.connections.get_mut(&source_id) {
            // port exists,
            Some(dest_connections) => {
                // check if dest processor exists
                match self.processors.get(dest_id.0) {
                    // dest processor exists
                    Some(dest_processor) => {
                        // check if dest port exists
                        if dest_processor.inputs_amt() <= dest_id.1 {
                            return Err("Destination Port does not Exist".to_string());
                        }
                    }
                    // dest processor does not exist
                    _ => {
                        return Err("Destination Processor does not exist".to_string());
                    }
                }
                // connection is added
                dest_connections.insert(dest_id);
            }
            // src port does not exist
            None => {
                return Err("Source Processor or Processor Port does not exist".to_string());
            }
        }
        match self.get_topological_sorting() {
            Some(sorted) => {
                self.topological_sorting = sorted;
            }
            None => {
                return Err("Cycle detected".to_string());
            }
        }

        Ok(())
    }

    /// Values get passed along in the graph.
    fn process_graph(&mut self) {
        // clear input and output buffers
        for i in 0..self.processors.len() {
            self.input_buffers[i] = 
                empty_buffer(self.processors[i].inputs_amt(), self.buffersize);
            self.output_buffers[i] =
                empty_buffer(self.processors[i].outputs_amt(), self.buffersize);
        }

        // pass graph input buffers to connected Processors
        // iterate over all graph input connections
        for (src, dest) in &self.input_connections {
            // iterate over all destination input ports
            for &(dest_proc, dest_port) in dest {
                // iterate over all samples
                for sample in 0..self.buffersize {
                    self.input_buffers[dest_proc][sample][dest_port] =
                        self.graph_input_buffers[sample][*src];
                }
            }
        }

        // go through the sorted processors and pass the Frames on
        for src_processor in &self.topological_sorting {
            self.processors[*src_processor].process(
                &self.input_buffers[*src_processor],
                &mut self.output_buffers[*src_processor],
            );
            // iterate over output ports
            for src_port in 0..self.processors[*src_processor].outputs_amt() {
                // match for connected inputs
                if let Some(connected_ports) = self.connections.get(&(*src_processor, src_port)) {
                    // iterate over connected inputs
                    for &(dest_processor, dest_port) in connected_ports {
                        // iterate over samples
                        for sample in 0..self.buffersize {
                            self.input_buffers[dest_processor][sample][dest_port] =
                                self.input_buffers[dest_processor][sample][dest_port].zip_map(
                                    self.output_buffers[*src_processor][sample][src_port],
                                    |x, y| x.add_amp(y.to_sample()),
                                );
                        }
                    }
                }
            }
        }

        // pass data to graph output buffers
        for (dest, src) in &self.output_connections {
            for &(src_proc, src_port) in src {
                for sample in 0..self.buffersize {
                    self.graph_output_buffers[sample][*dest] =
                        self.graph_output_buffers[sample][*dest].zip_map(
                            self.output_buffers[src_proc][sample][src_port],
                            |x, y| x.add_amp(y.to_sample()),
                        );
                }
            }
        }
    }

    /// returns the topological sorting of the graph in case there is no cycle
    pub fn get_topological_sorting(&self) -> Option<Vec<usize>> {
        let mut petgraph: PetGraph<(), (), petgraph::Directed, u32> = PetGraph::new();
        let mut pet_ix_to_graph_ix = HashMap::new();
        let mut graph_ix_to_pet_ix = HashMap::new();
        for i in 0..self.processors.len() {
            let petgraph_index = petgraph.add_node(());
            graph_ix_to_pet_ix.insert(i, petgraph_index);
            pet_ix_to_graph_ix.insert(petgraph_index, i);
        }

        for (&(src_processor, _), in_port_ids) in &self.connections {
            for &(dest_processor, _) in in_port_ids {
                petgraph.add_edge(
                    graph_ix_to_pet_ix[&src_processor],
                    graph_ix_to_pet_ix[&dest_processor],
                    (),
                );
            }
        }

        match petgraph::algo::toposort(&petgraph, None) {
            Ok(sorted) => {
                let mut result = Vec::new();
                for s in sorted {
                    result.push(pet_ix_to_graph_ix[&s]);
                }
                return Some(result);
            }
            Err(_) => return None,
        }
    }

    pub fn get_description_string(&self) -> String {
        let mut string = String::new();
        string += &format!("Processors: {}\n", self.processors.len());
        string += "Connections: \n";
        for (&(src_proc, src_port), dest_procs) in &self.connections {
            string += &format!("\tsrc Processor: {}, src Port: {}\n", src_proc, src_port);
            for &(dest_proc, dest_port) in dest_procs {
                string += &format!(
                    "\t\tdest Processor: {}, dest Port: {}\n",
                    dest_proc,
                    dest_port
                );
            }
        }
        string
    }

    fn inport_exists(&self, port: PortId) -> bool {
        if port.0 < self.processors.len() {
            if port.1 < self.processors[port.0].inputs_amt() {
                return true;
            }
        }
        false
    }

    fn outport_exists(&self, port: PortId) -> bool {
        if port.0 < self.processors.len() {
            if port.1 < self.processors[port.0].outputs_amt() {
                return true;
            }
        }
        false
    }
}

impl<F> Processor<F> for Graph<F>
where
    F: Frame,
{
    /// takes an list of input Frames and output Frames,
    /// processes the input and writes it to the outputs list.
    fn process(&mut self, inputs: &BufferSet<F>, outputs: &mut BufferSet<F>) {
        self.graph_output_buffers = empty_buffer(self.graph_output_buffers[0].len(), self.buffersize);
        for i in 0..inputs.len() {
            for j in 0..inputs[i].len() {
                self.graph_input_buffers[i][j] = inputs[i][j];
            }
        }
        self.process_graph();
        for i in 0..outputs.len() {
            for j in 0..outputs[i].len() {
                outputs[i][j] = self.graph_output_buffers[i][j];
            }
        }
    }

    /// returns the amount of inputs
    fn inputs_amt(&self) -> usize {
        self.input_buffers.len()
    }

    /// returns the amount of outputs
    fn outputs_amt(&self) -> usize {
        self.output_buffers.len()
    }
}

fn empty_buffer<F>(inner_size: usize, outer_size: usize) -> BufferSet<F>
where
    F: Frame,
{
    vec![vec![F::equilibrium(); inner_size]; outer_size]
}
