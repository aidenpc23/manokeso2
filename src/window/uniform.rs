use crate::camera::Camera;

use crate::window::state::GameWindow;

const DEFAULT_SCALE: f32 = 0.05;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pos: [f32; 2],
    proj: [f32; 2],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            pos: [0., 0.],
            proj: [1., 1.],
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, size: &[u32; 2]) {
        self.pos = camera.pos;
        let win_aspect = size[0] as f32 / size[1] as f32;
        self.proj = if win_aspect > camera.aspect {
            [1.0, win_aspect]
        } else {
            [camera.aspect/win_aspect, camera.aspect]
        };
        self.proj[0] *= camera.scale * DEFAULT_SCALE;
        self.proj[1] *= camera.scale * DEFAULT_SCALE;
    }
}

impl GameWindow {
    pub fn update_view(&mut self, camera: &Camera) {
        let size = self.window.inner_size();
        self.camera_uniform.update_view_proj(&camera, &[size.width, size.height]);
        self.queue.write_buffer(
            &self.buffer.camera,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}
