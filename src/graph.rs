extern crate petgraph;
extern crate sample;

use processor::Processor;
use self::sample::{Frame, Sample};
use std::collections::HashMap;
use std::collections::HashSet;
use self::petgraph::graph::Graph as PetGraph;

type PortId = (usize, usize);

/// The main container struct for Processors.
/// Processors can be added and connected in arbitrary
/// ways as long there are no cyclic connections.
/// All Processors must be from the same Frame type.
/// A graph has an arbitrary number of inputs and outputs
/// which can be connected to processors.
/// These inputs and outputs are called graph inputs and graph outputs.
pub struct Graph<F: Frame> {
    // contains all processors
    processors: Vec<Box<Processor<F>>>,
    // buffers that contains the graph inputs
    graph_input_buffers: Vec<F>,
    // buffers that contain the graph outputs
    graph_output_buffers: Vec<F>,
    // input buffers for all processors
    input_buffers: Vec<Vec<F>>,
    // output buffers for all processors
    output_buffers: Vec<Vec<F>>,
    // a hash map describing all connections from port to port
    connections: HashMap<PortId, HashSet<PortId>>,
    // a list of connections from the inputs to nodes
    input_connections: HashMap<usize, HashSet<PortId>>,
    // a list of connections from nodes to the outputs
    output_connections: HashMap<usize, HashSet<PortId>>,
    // stores all processor indexes sorted topologically
    topological_sorting: Vec<usize>,
}

impl<F> Graph<F>
where
    F: Frame,
{
    /// Create a new empty Graph
    pub fn new() -> Self {
        Graph {
            processors: Vec::new(),
            graph_input_buffers: Vec::new(),
            graph_output_buffers: Vec::new(),
            input_connections: HashMap::new(),
            output_connections: HashMap::new(),
            connections: HashMap::new(),
            topological_sorting: Vec::new(),
            input_buffers: Vec::new(),
            output_buffers: Vec::new(),
        }
    }

    /// Add a new processor to the Graph. Its ID gets returned.
    pub fn add_processor(&mut self, processor: Box<Processor<F>>) -> usize {
        let index = self.processors.len();
        self.input_buffers
            .push(vec![F::equilibrium(); processor.inputs_amt()]);
        self.output_buffers
            .push(vec![F::equilibrium(); processor.outputs_amt()]);
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
            }, 
            None => {
                Err(format!("input {} does not exist", input))
            }
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
            }, 
            None => {
                Err(format!("input {} does not exist", output))
            }
        }
    }

    /// set the amount of inputs
    pub fn set_input_amt(&mut self, inputs: usize) {
        self.graph_input_buffers = vec![F::equilibrium(); inputs];
        self.input_connections = HashMap::new();
        for i in 0..inputs {
            self.input_connections.insert(i, HashSet::new());
        }
    }

    /// set the amount of outputs
    pub fn set_output_amt(&mut self, outputs: usize) {
        self.graph_output_buffers = vec![F::equilibrium(); outputs];
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
            self.input_buffers[i] = vec![F::equilibrium(); self.processors[i].inputs_amt()];
            self.output_buffers[i] = vec![F::equilibrium(); self.processors[i].outputs_amt()];
        }

        // pass graph input buffers to connected Processors
        for (src, dest) in &self.input_connections {
            for &(dest_proc, dest_port) in dest {
                self.input_buffers[dest_proc][dest_port] = self.graph_input_buffers[*src];
            }
        }

        // go through the sorted processors and pass the Frames on
        for processor in &self.topological_sorting {
            self.processors[*processor].process(
                &mut self.input_buffers[*processor],
                &mut self.output_buffers[*processor],
            );
            for output in 0..self.processors[*processor].outputs_amt() {
                if let Some(connected_ports) = self.connections.get(&(*processor, output)) {
                    for &(input_processor, input_port) in connected_ports {
                        let new_input = self.input_buffers[input_processor][input_port]
                                .zip_map(self.output_buffers[*processor][output],
                                         |x, y| {
                                             x.add_amp(y.to_sample())
                                         });
                        self.input_buffers[input_processor][input_port] = new_input;
                    }
                }
            }
        }

        // pass data to graph output buffers
        for (dest, src) in &self.output_connections {
            for &(src_proc, src_port) in src {
                let mut new_output: F;
                {
                    new_output = self.output_buffers[src_proc][src_port]
                        .zip_map(self.graph_output_buffers[*dest],
                                 |x, y| {
                                     x.add_amp(y.to_sample())
                                 })
                                  
                }
                self.graph_output_buffers[*dest] = new_output;
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
    fn process(&mut self, inputs: &mut Vec<F>, outputs: &mut Vec<F>) {
        for i in 0..inputs.len() {
            self.graph_input_buffers[i] = inputs[i];
        }
        self.process_graph();
        for i in 0..outputs.len() {
            outputs[i] = self.graph_output_buffers[i];
        }
    }

    /// returns the amount of inputs
    fn inputs_amt(&self) -> usize {
        self.input_buffers.len()
    }

    /// returns the amount of outputs
    fn outputs_amt(&self) -> usize {
        self.input_buffers.len()
    }
}
