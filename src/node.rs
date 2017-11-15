extern crate sample;

pub trait Node<T: sample::Frame> {
    fn process(&mut self, &mut Vec<T>, &mut Vec<T>);
    fn inputs_amt(&self) -> usize;
    fn outputs_amt(&self) -> usize;
}
