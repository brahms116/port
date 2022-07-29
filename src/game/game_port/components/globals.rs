use super::*;

pub struct Player();

pub struct StoryBlocker();

pub struct InputController {
    down_point: Option<Vec2>,
}

impl InputController {
    pub fn new() -> Self {
        Self { down_point: None }
    }

    pub fn is_mouse_down(&self) -> bool {
        self.down_point.is_some()
    }

    pub fn mouse_down(&mut self, point: Vec2) {
        self.down_point = Some(point);
    }

    pub fn mouse_up(&mut self) {
        self.down_point = None;
    }

    pub fn input(&self, curr: Vec2) -> GameInput {
        let dead_zone = 0.0;
        if let Some(pt) = self.down_point {
            GameInput {
                up: curr.y - pt.y > dead_zone,
                left: pt.x - curr.x > dead_zone,
                right: curr.x - pt.x > dead_zone,
                down: pt.y - curr.y > dead_zone,
            }
        } else {
            GameInput::default()
        }
    }
}
