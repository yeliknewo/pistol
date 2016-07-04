pub struct Transform {
    scale: (f64, f64),
    position: (f64, f64),
    layer: u8,
}

impl_component!(Transform, false, false);

impl Transform {
    pub fn new_default() -> Transform {
        Transform::new((1.0, 1.0), (0.0, 0.0), 0)
    }

    pub fn new(scale: (f64, f64), position: (f64, f64), layer: u8) -> Transform {
        Transform {
            scale: scale,
            position: position,
            layer: layer,
        }
    }

    pub fn set_position(&mut self, position: (f64, f64)) {
        self.position = position;
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn set_scale(&mut self, scale: (f64, f64)) {
        self.scale = scale;
    }

    pub fn get_scale(&self) -> (f64, f64) {
        self.scale
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.layer = layer;
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }
}
