use super::*;

#[derive(Debug, Default, Clone)]
pub struct GameInput {
    pub horizontal: f64,
    pub vertical: f64,
}

pub trait GameApi {
    fn window_size(&self) -> &WindowSize;
    fn draw_surface(&self, surface: Surface);
    fn set_element_position(&self, point: Vec2, id: &str);
    fn set_element_rotation(&self, deg: f64, id: &str);
    fn resize_element(&self, size: u32, id: &str);
    fn set_element_opacity(&self, opacity: u32, id: &str);
    fn inputs(&self) -> &MouseInput;
    fn key_inputs(&self) -> &GameInput;
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

#[derive(Clone)]
pub struct WindowSize {
    pub w: i32,
    pub h: i32,
}

pub fn map_vec2(
    a: &Vec2,
    center: &Vec2,
    window_size: &WindowSize,
) -> Vec2 {
    let window_center = Vec2::new(
        window_size.w as f64,
        window_size.h as f64,
    ) * 0.5;
    let diff_vec = *a - *center;
    /* Invert the y axis here, so that positive values go "up" */
    let screen_pos = Vec2::new(
        window_center.x + diff_vec.x,
        window_center.y - diff_vec.y,
    );
    screen_pos
}
