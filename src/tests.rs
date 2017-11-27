extern crate sample;

#[cfg(test)]
mod tests {

    use super::super::graph::Graph;
    use super::super::graph::BufferSet;
    use super::super::processor::Processor;

    struct TestProcessor {}

    impl super::super::processor::Processor<[f32; 2]> for TestProcessor {
        fn process(&mut self, inputs: &BufferSet<[f32; 2]>, outputs: &mut BufferSet<[f32; 2]>) {
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



    #[test]
    fn cyclic_graph_test_1() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n1, 0), &mut (n4, 0)).unwrap();
    }

    #[test]
    fn cyclic_graph_test_2() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n1, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n1, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n4, 0)).unwrap();
    }

    #[test]
    fn cyclic_graph_test_3() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n4, 0)).unwrap();
        match graph.add_connection(&mut (n4, 0), &mut (n1, 0)) {
            Ok(_) => {
                panic!();
            }
            _ => {}
        }
    }

    #[test]
    fn cyclic_graph_test_4() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n1, 0), &mut (n3, 0)).unwrap();
        match graph.add_connection(&mut (n4, 0), &mut (n1, 0)) {
            Ok(_) => {
                panic!();
            }
            _ => {}
        }
    }

    #[test]
    fn topologic_sorting_test_1() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n4, 0)).unwrap();
        assert_eq!(graph.get_topological_sorting(), Some(vec![n1, n2, n3, n4]));
    }

    #[test]
    fn topologic_sorting_test_2() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        let n5 = graph.add_processor(Box::new(TestProcessor {}));
        let n6 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n6, 0), &mut (n5, 0)).unwrap();
        graph.add_connection(&mut (n6, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n6, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n6, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n6, 0), &mut (n1, 0)).unwrap();
        graph.add_connection(&mut (n5, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n5, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n5, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n5, 0), &mut (n1, 0)).unwrap();
        graph.add_connection(&mut (n4, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n4, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n4, 0), &mut (n1, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n1, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n1, 0)).unwrap();
        assert_eq!(
            graph.get_topological_sorting(),
            Some(vec![n6, n5, n4, n3, n2, n1])
        );
    }

    #[test]
    fn pass_through_test_1() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n4, 0)).unwrap();
        graph.set_input_amt(1);
        graph.set_output_amt(1);
        graph.connect_input(0, (n1, 0)).unwrap();
        graph.connect_output(0, (n4, 0)).unwrap();
        let mut output_buffer: Vec<Vec<[f32; 2]>> = vec![vec![[0.0, 0.0]]];
        let      input_buffer: Vec<Vec<[f32; 2]>> = vec![vec![[4.1, 6.2]]];
        Processor::process(&mut graph, &input_buffer, &mut output_buffer);
        assert_eq!(input_buffer[0][0][0], output_buffer[0][0][0]);
        assert_eq!(input_buffer[0][0][1], output_buffer[0][0][1]);
    }

    #[test]
    fn pass_through_test_2() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n3, 0)).unwrap();
        graph.set_input_amt(2);
        graph.set_output_amt(1);
        graph.connect_input(0, (n1, 0)).unwrap();
        graph.connect_input(1, (n2, 0)).unwrap();
        graph.connect_output(0, (n3, 0)).unwrap();
        let mut output_buffer: Vec<Vec<[f32; 2]>> = vec![vec![[0.4, 0.7]]];
        let      input_buffer: Vec<Vec<[f32; 2]>> = vec![vec![[0.1, 0.2]],vec![[0.3, 0.5]]];
        Processor::process(&mut graph, &input_buffer, &mut output_buffer);
        assert_eq!(input_buffer[0][0][0] + input_buffer[1][0][0], output_buffer[0][0][0]);
        assert_eq!(input_buffer[0][0][1] + input_buffer[1][0][1], output_buffer[0][0][1]);
    }

    #[test]
    fn pass_through_test_3() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        let n3 = graph.add_processor(Box::new(TestProcessor {}));
        let n4 = graph.add_processor(Box::new(TestProcessor {}));
        graph.add_connection(&mut (n1, 0), &mut (n2, 0)).unwrap();
        graph.add_connection(&mut (n1, 0), &mut (n3, 0)).unwrap();
        graph.add_connection(&mut (n3, 0), &mut (n4, 0)).unwrap();
        graph.add_connection(&mut (n2, 0), &mut (n4, 0)).unwrap();
        graph.set_input_amt(1);
        graph.set_output_amt(1);
        graph.connect_input(0, (n1, 0)).unwrap();
        graph.connect_output(0, (n4, 0)).unwrap();
        let mut output_buffer: Vec<Vec<[f32; 2]>> = vec![vec![[0.4, 0.7]]];
        let      input_buffer: Vec<Vec<[f32; 2]>> = vec![vec![[0.4, 0.7]]];
        Processor::process(&mut graph, &input_buffer, &mut output_buffer);
        assert_eq!(input_buffer[0][0][0] * 2.0, output_buffer[0][0][0]);
        assert_eq!(input_buffer[0][0][1] * 2.0, output_buffer[0][0][1]);
    }

    #[test]
    fn too_much_connections_test() {
        let mut graph = Graph::<[f32; 2]>::new(1);
        let n1 = graph.add_processor(Box::new(TestProcessor {}));
        let n2 = graph.add_processor(Box::new(TestProcessor {}));
        match graph.add_connection(&mut (n1, 1), &mut (n2, 0)) {
            Ok(_) => {
                panic!();
            }
            _ => {}
        }
        match graph.add_connection(&mut (n1, 0), &mut (n2, 1)) {
            Ok(_) => {
                panic!();
            }
            _ => {}
        }
        match graph.add_connection(&mut (n1, 0), &mut (n1, 0)) {
            Ok(_) => {
                panic!();
            }
            _ => {}
        }
    }
}
