extern crate sample;

pub struct TestProcessor {}

impl super::processor::Processor<[f32; 2]> for TestProcessor {
    fn process(&mut self, inputs: &mut Vec<[f32; 2]>, outputs: &mut Vec<[f32; 2]>) {
        for i in 0..inputs.len() {
            for j in 0..inputs[i].len() {
                outputs[i][j] = inputs[i][j];
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

#[cfg(test)]
mod tests {

    use super::super::graph::Graph;
    use super::super::processor::Processor;

    #[test]
    fn cyclic_graph_test_1() {
        let mut graph = Graph::<[f32; 2]>::new();
        let n1 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n2 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n3 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n4 = graph.add_processor(Box::new(super::TestProcessor {}));
        graph.add_connection(n1, 0, n2, 0).unwrap();
        graph.add_connection(n2, 0, n3, 0).unwrap();
        graph.add_connection(n3, 0, n4, 0).unwrap();
        graph.add_connection(n1, 0, n4, 0).unwrap();
    }

    #[test]
    fn cyclic_graph_test_2() {
        let mut graph = Graph::<[f32; 2]>::new();
        let n1 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n2 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n3 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n4 = graph.add_processor(Box::new(super::TestProcessor {}));
        graph.add_connection(n1, 0, n2, 0).unwrap();
        graph.add_connection(n1, 0, n3, 0).unwrap();
        graph.add_connection(n1, 0, n4, 0).unwrap();
        graph.add_connection(n2, 0, n4, 0).unwrap();
    }

    #[test]
    fn cyclic_graph_test_3() {
        let mut graph = Graph::<[f32; 2]>::new();
        let n1 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n2 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n3 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n4 = graph.add_processor(Box::new(super::TestProcessor {}));
        graph.add_connection(n1, 0, n2, 0).unwrap();
        graph.add_connection(n2, 0, n3, 0).unwrap();
        graph.add_connection(n3, 0, n4, 0).unwrap();
        match graph.add_connection(n4, 0, n1, 0) {
            Ok(_) => {
                panic!();
            }
            _ => {}
        }
    }

    #[test]
    fn cyclic_graph_test_4() {
        let mut graph = Graph::<[f32; 2]>::new();
        let n1 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n2 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n3 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n4 = graph.add_processor(Box::new(super::TestProcessor {}));
        graph.add_connection(n1, 0, n2, 0).unwrap();
        graph.add_connection(n2, 0, n3, 0).unwrap();
        graph.add_connection(n3, 0, n4, 0).unwrap();
        graph.add_connection(n2, 0, n4, 0).unwrap();
        graph.add_connection(n1, 0, n3, 0).unwrap();
        match graph.add_connection(n4, 0, n1, 0) {
            Ok(_) => {
                panic!();
            }
            _ => {}
        }
    }

    #[test]
    fn topologic_sorting_test_1() {
        let mut graph = Graph::<[f32; 2]>::new();
        let n1 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n2 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n3 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n4 = graph.add_processor(Box::new(super::TestProcessor {}));
        graph.add_connection(n1, 0, n2, 0).unwrap();
        graph.add_connection(n2, 0, n3, 0).unwrap();
        graph.add_connection(n3, 0, n4, 0).unwrap();
        graph.add_connection(n2, 0, n4, 0).unwrap();
        assert_eq!(graph.get_topological_sorting(), Some(vec![n1, n2, n3, n4]));
    }

    #[test]
    fn topologic_sorting_test_2() {
        let mut graph = Graph::<[f32; 2]>::new();
        let n1 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n2 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n3 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n4 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n5 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n6 = graph.add_processor(Box::new(super::TestProcessor {}));
        graph.add_connection(n6, 0, n5, 0).unwrap();
        graph.add_connection(n6, 0, n4, 0).unwrap();
        graph.add_connection(n6, 0, n3, 0).unwrap();
        graph.add_connection(n6, 0, n2, 0).unwrap();
        graph.add_connection(n6, 0, n1, 0).unwrap();
        graph.add_connection(n5, 0, n4, 0).unwrap();
        graph.add_connection(n5, 0, n3, 0).unwrap();
        graph.add_connection(n5, 0, n2, 0).unwrap();
        graph.add_connection(n5, 0, n1, 0).unwrap();
        graph.add_connection(n4, 0, n3, 0).unwrap();
        graph.add_connection(n4, 0, n2, 0).unwrap();
        graph.add_connection(n4, 0, n1, 0).unwrap();
        graph.add_connection(n3, 0, n2, 0).unwrap();
        graph.add_connection(n3, 0, n1, 0).unwrap();
        graph.add_connection(n2, 0, n1, 0).unwrap();
        assert_eq!(
            graph.get_topological_sorting(),
            Some(vec![n6, n5, n4, n3, n2, n1])
        );
    }

    #[test]
    fn pass_through_test() {
        let mut graph = Graph::<[f32; 2]>::new();
        let n1 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n2 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n3 = graph.add_processor(Box::new(super::TestProcessor {}));
        let n4 = graph.add_processor(Box::new(super::TestProcessor {}));
        graph.add_connection(n1, 0, n2, 0).unwrap();
        graph.add_connection(n2, 0, n3, 0).unwrap();
        graph.add_connection(n3, 0, n4, 0).unwrap();
        graph.set_input_amt(1);
        graph.set_output_amt(1);
        graph.connect_input(0, (n1, 0));
        graph.connect_output(0, (n4, 0));
        let mut output_buffer = vec![[0.0, 0.0]];
        let mut input_buffer = vec![[4.1, 6.2]];
        Processor::process(&mut graph, &mut input_buffer, &mut output_buffer);
        assert_eq!(input_buffer[0][0], output_buffer[0][0]);
        assert_eq!(input_buffer[0][1], output_buffer[0][1]);
    }
}
