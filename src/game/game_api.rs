use super::*;

pub trait GameApi {
    fn window_size(&self) -> &WindowSize;
    fn draw_surface(&self, surface: Surface);
    fn set_element_position(&self, point: Vec2, id: &str);
    fn resize_element(&self, size: u32, id: &str);
    fn set_element_opacity(&self, opacity: u32, id: &str);
    fn inputs(&self) -> &MouseInput;
    fn log(&self, msg: &str);
}

#[derive(Clone)]
pub struct MouseInput {
    pub pos: Vec2,
    pub is_down: bool,
}

impl Default for MouseInput {
    fn default() -> Self {
        MouseInput {
            is_down: false,
            pos: Vec2::default(),
        }
    }
}

pub struct WindowSize {
    pub w: i32,
    pub h: i32,
}

pub fn map_vec2(a: &Vec2, center: &Vec2, window_size: &WindowSize) -> Vec2 {
    todo!()
}
