pub trait AudioSource: Send {
    fn next_sample(&mut self) -> f32;
}
