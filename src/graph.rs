extern crate sample;

use node::Node;
use self::sample::Frame;
use std::marker::PhantomData;

pub struct Graph<F: Frame, N: Node<F>> {
    nodes: Vec<Box<N>>,
    connections: Vec<(usize, usize, usize, usize)>,
    _marker: PhantomData<F>,
}

impl<F, N> Graph<F, N> 
    where F: Frame, N: Node<F> {

    /// Add a new node to the Graph. Its ID gets returned.
    pub fn add_node(&mut self, node: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Box::new(node));
        return index;
    }

    pub fn connect_nodes(&mut self,
                         input_node: usize, 
                         input_port: usize, 
                         output_node: usize, 
                         output_port: usize) -> Result<usize, String> {
        let input_ports  = (*self.nodes[input_node]).inputs_amt();
        let output_ports = self.nodes[input_node].outputs_amt();
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
}
