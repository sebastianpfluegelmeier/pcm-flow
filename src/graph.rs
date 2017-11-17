extern crate petgraph;
extern crate sample;

use processor::Processor;
use self::sample::Frame;
use std::collections::HashMap;
use self::petgraph::graph::Graph as PetGraph;

type Connection = (PortId, PortId);
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
    // a list of all connections between the processors
    // TODO: remove and use outputs instead
    connections: Vec<Connection>,
    // a hash map describing all connections from port to port
    // TODO: change to HashMap<PortId, HashSet<PortId>>
    outputs: HashMap<PortId, PortId>,
    // a list of connections from the inputs to nodes
    // TODO: change to HashMap<usize, HashSet<PortId>>
    input_connections: Vec<PortId>,
    // a list of connections from nodes to the outputs
    // TODO: change to HashMap<usize, HashSet<PortId>>
    output_connections: Vec<PortId>,

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
            connections: Vec::new(),
            input_connections: Vec::new(),
            output_connections: Vec::new(),
            outputs: HashMap::new(),
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
        self.processors.push(processor);
        return index;
    }

    /// Connect an input to a processor.
    pub fn connect_input(&mut self, input: usize, port: PortId) {
        self.input_connections[input] = port;
    }

    /// connect an output to a processor.
    pub fn connect_output(&mut self, output: usize, port: PortId) {
        self.output_connections[output] = port;
    }

    /// set the amount of inputs
    pub fn set_input_amt(&mut self, inputs: usize) {
        self.graph_input_buffers = vec![F::equilibrium(); inputs];
        self.input_connections = vec![(0, 0); inputs];
    }

    /// set the amount of outputs
    pub fn set_output_amt(&mut self, outputs: usize) {
        self.graph_output_buffers = vec![F::equilibrium(); outputs];
        self.output_connections = vec![(0, 0); outputs];
    }

    /// add aconnection between two ports
    /// either returns an Ok(connection Id) or in case of a cycle or an invalid
    /// port, a Err(Description)
    pub fn add_connection(
        &mut self,
        // TODO: change to PortId
        input_processor: usize,
        input_port: usize,
        // TODO: change to PortId
        output_processor: usize,
        output_port: usize,
    ) -> Result<usize, String> {
        let input_ports_amt = (*self.processors[input_processor]).inputs_amt();
        let output_ports_amt = self.processors[input_processor].outputs_amt();

        self.connections
            .push(((input_processor, input_port), (output_processor, output_port)));
        if let Some(topo_sorting) = self.get_topological_sorting() {
            self.topological_sorting = topo_sorting;
        } else {
            self.connections.pop();
            return Err(format!(
                "Cycle found after Connecting Processor {} and Processor {}",
                input_processor,
                output_processor
            ));
        }

        if input_port < input_ports_amt && output_port < output_ports_amt {
            let connections_len = self.connections.len();
            self.outputs
                .insert((input_processor, input_port), (output_processor, output_port));
            return Ok(connections_len);
        } else if input_port < input_ports_amt {
            self.connections.pop();
            return Err(format!(
                "Port {} does not exist on Processor {}",
                input_port,
                input_processor
            ));
        } else {
            self.connections.pop();
            return Err(format!(
                "Port {} does not exist on Processor {}",
                output_port,
                output_port
            ));
        }
    }

    /// Values get passed along in the graph.
    fn process_graph(&mut self) {
        // clear input and output buffers
        for i in 0..self.processors.len() {
            self.input_buffers[i] = vec![F::equilibrium(); self.processors[i].inputs_amt()];
            self.output_buffers[i] = vec![F::equilibrium(); self.processors[i].outputs_amt()];
        }

        // pass graph input buffers to connected Processors
        for (input, &(processor, port)) in self.input_connections.iter().enumerate() {
            self.input_buffers[processor][port] = self.graph_input_buffers[input];
        }

        // go through the sorted processors and pass the Frames on
        for processor in &self.topological_sorting {
            self.processors[*processor].process(
                &mut self.input_buffers[*processor],
                &mut self.output_buffers[*processor],
            );
            for output in 0..self.processors[*processor].outputs_amt() {
                if let Some(&(input_processor, input_port)) = self.outputs.get(&(*processor, output)) {
                    self.input_buffers[input_processor][input_port] 
                        = self.output_buffers[*processor][output];
                }
            }
        }

        // pass data to graph output buffers
        for (output, &(processor, port)) in self.output_connections.iter().enumerate() {
            self.graph_output_buffers[output] = self.output_buffers[processor][port];
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

        for &((s, _), (e, _)) in &self.connections {
            petgraph.add_edge(graph_ix_to_pet_ix[&s], graph_ix_to_pet_ix[&e], ());
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
