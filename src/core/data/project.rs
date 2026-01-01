use crate::core::data::layer::Layer;

pub struct Project {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl Project {
    pub fn new(width: usize, height: usize) -> Self {
        Project {
            width,
            height,
            layers: vec![],
        }
    }

    pub fn add_layer(&mut self, name: String) {
        self.layers.push(Layer::new(name, self.width, self.height));
    }
}