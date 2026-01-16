use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

#[derive(Clone)]
pub struct Parameter {
    value: Arc<AtomicU32>,
}

impl Parameter {
    pub fn new(value: f32) -> Self {
        Self {
            value: Arc::new(AtomicU32::new(value.to_bits())),
        }
    }

    pub fn set(&self, new_value: f32) {
        self.value.store(new_value.to_bits(), Ordering::Relaxed);
    }

    pub fn get(&self) -> f32 {
        let bits = self.value.load(Ordering::Relaxed);
        f32::from_bits(bits)
    }
}
