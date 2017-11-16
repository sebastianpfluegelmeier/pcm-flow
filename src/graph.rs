extern crate sample;
extern crate petgraph;

use node::Node;
use self::sample::Frame;
use std::collections::HashMap;
use self::petgraph::graph::Graph as PetGraph;

pub struct Graph<F: Frame> {
    nodes: Vec<Box<Node<F>>>,
    graph_input_buffers: Vec<F>,
    graph_output_buffers: Vec<F>,
    input_buffers: Vec<Vec<F>>,
    output_buffers: Vec<Vec<F>>,
    // input_node, input_port, output_node, output_port
    connections: Vec<(usize, usize, usize, usize)>,
    // input_port<(node, port)>
    input_connections: Vec<(usize, usize)>,
    // output_port<(node, port)>
    output_connections: Vec<(usize, usize)>,
    outputs: HashMap<(usize, usize), (usize, usize)>,
    topological_sorting: Vec<usize>,
}

impl<F> Graph<F> 
    where F: Frame {

    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            graph_input_buffers: Vec::new(),
            graph_output_buffers: Vec::new(),
            connections: Vec::new(),
            input_connections: Vec::new(),
            output_connections: Vec::new(),
            outputs: HashMap::new(),
            topological_sorting: Vec::new(),
            input_buffers: Vec::new(),
            output_buffers: Vec::new()
        }
    }

    /// Add a new node to the Graph. Its ID gets returned.
    pub fn add_node(&mut self, node: Box<Node<F>>) -> usize {
        let index = self.nodes.len();
        self.input_buffers .push(vec![F::equilibrium(); node.inputs_amt() ]);
        self.output_buffers.push(vec![F::equilibrium(); node.outputs_amt()]);
        self.nodes.push(node);
        return index;
    }

    pub fn connect_input(&mut self, input: usize, node: usize, port: usize) {
        self.input_connections[input] = (node, port);
    }

    pub fn connect_output(&mut self, output: usize, node: usize, port: usize) {
        self.output_connections[output] = (node, port);
    }

    pub fn set_input_amt(&mut self, inputs: usize) {
        self.graph_input_buffers = vec![F::equilibrium(); inputs];
    }

    pub fn set_output_amt(&mut self, outputs: usize) {
        self.graph_output_buffers = vec![F::equilibrium(); outputs];
    }

    pub fn add_connection(&mut self,
                         input_node: usize, 
                         input_port: usize, 
                         output_node: usize, 
                         output_port: usize) -> Result<usize, String> {
        self.connections.push((input_node, input_port, output_node, output_port));
        if let None = self.get_topological_sorting() {
            return Err(format!("Found a Cycle when trying to connect node {} and node {}",
                       input_node, output_node));
        }
        self.outputs.insert((input_node, input_port), (output_node, output_port));
        self.connections.pop();
        let input_ports  = (*self.nodes[input_node]).inputs_amt();
        let output_ports = self.nodes[input_node].outputs_amt();
        if let Some(topo_sorting) = self.get_topological_sorting() {
            self.topological_sorting = topo_sorting;
        } else {
            self.connections.pop();
            return Err(format!("Cycle found after Connecting Node {} and Node {}", 
                       input_node, output_node));
        }
        if input_port < input_ports && output_port < output_ports {
            let connections_len = self.connections.len();
            self.connections.push((input_node, input_port, output_node, output_port));
            return Ok(connections_len);
        } else if input_port < input_ports {
            return Err(format!("Port {} does not exist on Node {}", input_port, input_node));
        } else {
            return Err(format!("Port {} does not exist on Node {}", output_port, output_port));
        }
    }

    pub fn process_graph(&mut self) {
        // clear input and output buffers
        for i in 0..self.nodes.len() {
            self.input_buffers[i]  = vec![F::equilibrium(); self.nodes[i].inputs_amt()];
            self.output_buffers[i] = vec![F::equilibrium(); self.nodes[i].outputs_amt()];
        }
        // pass graph input buffers to connected Nodes
        for (input, &(node, port)) in self.input_connections.iter().enumerate() {
            self.input_buffers[node][port] = self.graph_input_buffers[input];
        }

        // go through the sorted nodes and pass the Frames on
        for node in &self.topological_sorting {
            self.nodes[*node].process(&mut self.input_buffers[*node], 
                                      &mut self.output_buffers[*node]);
            for output in 0..self.nodes[*node].outputs_amt() {
                if let Some(&(input_node, input_port)) = self.outputs.get(&(*node, output)){
                    self.input_buffers[input_node][input_port] = 
                        self.output_buffers[*node][output];
                }
            }
        }

        // pass data to graph output buffers
        for (output, &(node, port)) in self.output_connections.iter().enumerate() {
            self.graph_output_buffers[output] = self.input_buffers[node][port];
        }
    }

    pub fn get_topological_sorting(&self) -> Option<Vec<usize>> {
        let mut petgraph: PetGraph<(), (), petgraph::Directed, u32> = PetGraph::new();
        let mut pet_is_to_graph_is = HashMap::new();
        let mut graph_is_to_pet_is = HashMap::new();
        for i in 0..self.nodes.len() {
            let petgraph_index = petgraph.add_node(());
            graph_is_to_pet_is.insert(i, petgraph_index);
            pet_is_to_graph_is.insert(petgraph_index, i);
        }

        for &(s, _, e, _) in &self.connections {
            petgraph.add_edge(graph_is_to_pet_is[&s], graph_is_to_pet_is[&e], ());
        }

        match petgraph::algo::toposort(&petgraph, None) {
            Ok(sorted) => {
                let mut result = Vec::new();
                for s in sorted {
                    result.push(pet_is_to_graph_is[&s]);
                }
                return Some(result);
            },
            Err(_) => {return None}
        }
    }
}

impl<F> Node<F> for Graph<F> where F: Frame {

    fn process(&mut self, inputs: &mut Vec<F>, outputs: &mut Vec<F>) {
        self.graph_input_buffers = inputs.clone();
        self.process_graph();
        for i in 0..outputs.len() {
            outputs[i] = self.graph_output_buffers[i];
        }
    }

    fn inputs_amt(&self) -> usize {
        0
    }

    fn outputs_amt(&self) -> usize {
        0
    }
}
