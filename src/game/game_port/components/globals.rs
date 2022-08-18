use super::*;

const DEADEZONE_COUNT: u32 = 20;

pub struct Player();

pub struct InputController {
    prev_point: Option<Vec2>,
    prev_game_input: GameInput,
    deadzone_count: u32,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            prev_point: None,
            prev_game_input: GameInput::default(),
            deadzone_count: 0,
        }
    }

    pub fn is_mouse_down(&self) -> bool {
        self.prev_point.is_some()
    }

    pub fn mouse_down(&mut self, point: Vec2) {
        self.prev_point = Some(point);
    }

    pub fn mouse_up(&mut self) {
        self.prev_point = None;
        self.deadzone_count = 0;
        self.prev_game_input = GameInput::default();
    }

    pub fn input(&mut self, curr: Vec2) -> GameInput {
        let dead_zone = 20.0;
        if let Some(pt) = self.prev_point {
            let diff_vec = curr - pt;
            if diff_vec.mag() < dead_zone {
                self.deadzone_count += 1;
            }
            if self.deadzone_count > DEADEZONE_COUNT {
                self.prev_point = Some(pt);
            }

            let is_up = curr.y - pt.y > dead_zone;
            // let is_down =
            GameInput {
                up: curr.y - pt.y > 0.0,
                left: pt.x - curr.x > dead_zone,
                right: curr.x - pt.x > dead_zone,
                down: pt.y - curr.y > 0.0,
            }
        } else {
            GameInput::default()
        }
    }
}
