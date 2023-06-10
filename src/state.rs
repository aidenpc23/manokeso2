use crate::camera::Camera;

pub struct GameState {
    pub camera: Camera,
    pub camera_scroll: f32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0
        }
    }
}
