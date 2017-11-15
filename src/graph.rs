extern crate sample;
extern crate petgraph;

use node::Node;
use self::sample::Frame;
use std::collections::HashMap;
use self::petgraph::graph::Graph as PetGraph;

pub struct Graph<F: Frame> {
    nodes: Vec<Box<Node<F>>>,
    // input_node, input_port, output_node, output_port
    connections: Vec<(usize, usize, usize, usize)>,
    topological_sorting: Vec<usize>
}

impl<F> Graph<F> 
    where F: Frame {

    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            connections: Vec::new(),
            topological_sorting: Vec::new()
        }
    }

    /// Add a new node to the Graph. Its ID gets returned.
    pub fn add_node(&mut self, node: Box<Node<F>>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        return index;
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
