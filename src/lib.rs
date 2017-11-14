extern crate sample;
pub mod node;
pub mod graph;

pub struct TestNode { }

impl self::node::Node<[f32; 2]> for TestNode {
    fn process(&mut self, input: Vec<[f32; 2]>) -> Vec<[f32; 2]> {
        input
    }
    fn inputs_amt(&self) -> usize { 1 }
    fn outputs_amt(&self) -> usize { 1 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cyclic_graph_test1() {
        let mut graph = super::graph::Graph::<[f32; 2], super::TestNode>::new();
        let n1 = graph.add_node(super::TestNode{});
        let n2 = graph.add_node(super::TestNode{});
        let n3 = graph.add_node(super::TestNode{});
        let n4 = graph.add_node(super::TestNode{});
        graph.add_connection(n1,0,n2,0).unwrap();
        graph.add_connection(n2,0,n3,0).unwrap();
        graph.add_connection(n3,0,n4,0).unwrap();
        graph.add_connection(n1,0,n4,0).unwrap();
    }

    #[test]
    fn cyclic_graph_test2() {
        let mut graph = super::graph::Graph::<[f32; 2], super::TestNode>::new();
        let n1 = graph.add_node(super::TestNode{});
        let n2 = graph.add_node(super::TestNode{});
        let n3 = graph.add_node(super::TestNode{});
        let n4 = graph.add_node(super::TestNode{});
        graph.add_connection(n1,0,n2,0).unwrap();
        graph.add_connection(n1,0,n3,0).unwrap();
        graph.add_connection(n1,0,n4,0).unwrap();
        graph.add_connection(n2,0,n4,0).unwrap();
    }
    
    #[test]
    fn cyclic_graph_test3() {
        let mut graph = super::graph::Graph::<[f32; 2], super::TestNode>::new();
        let n1 = graph.add_node(super::TestNode{});
        let n2 = graph.add_node(super::TestNode{});
        let n3 = graph.add_node(super::TestNode{});
        let n4 = graph.add_node(super::TestNode{});
        graph.add_connection(n1,0,n2,0).unwrap();
        graph.add_connection(n2,0,n3,0).unwrap();
        graph.add_connection(n3,0,n4,0).unwrap();
        match graph.add_connection(n4,0,n1,0) {
            Ok(_) => {
                panic!();
            },
            _ => {}
        }
    }

    #[test]
    fn cyclic_graph_test4() {
        let mut graph = super::graph::Graph::<[f32; 2], super::TestNode>::new();
        let n1 = graph.add_node(super::TestNode{});
        let n2 = graph.add_node(super::TestNode{});
        let n3 = graph.add_node(super::TestNode{});
        let n4 = graph.add_node(super::TestNode{});
        graph.add_connection(n1,0,n3,0).unwrap();
        graph.add_connection(n3,0,n2,0).unwrap();
        graph.add_connection(n2,0,n4,0).unwrap();
        graph.add_connection(n3,0,n4,0).unwrap();
        match graph.add_connection(n4,0,n1,0) {
            Ok(_) => {
                panic!();
            },
            _ => {}
        }
    }
}
