use super::*;

const DEADEZONE_COUNT: u32 = 2;

const STRAIGHT_RATIO: f64 = 1.0;

pub struct Player();

pub struct InputController {
    prev_dead_point: Option<Vec2>,
    prev_mouse_point: Vec2,
    prev_game_input: GameInput,
    deadzone_count: u32,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            prev_dead_point: None,
            prev_mouse_point: Vec2::default(),
            prev_game_input: GameInput::default(),
            deadzone_count: 0,
        }
    }

    pub fn is_mouse_down(&self) -> bool {
        self.prev_dead_point.is_some()
    }

    pub fn mouse_down(&mut self, point: Vec2) {
        self.prev_dead_point = Some(point);
    }

    pub fn mouse_up(&mut self) {
        self.prev_dead_point = None;
        self.deadzone_count = 0;
        self.prev_game_input = GameInput::default();
    }

    pub fn input(&mut self, curr: Vec2) -> GameInput {
        let dead_zone = 5.0;

        if let Some(pt) = self.prev_dead_point {
            let mut output = self.prev_game_input.clone();

            let diff_vec = curr - pt;
            let movt_vec = curr - self.prev_mouse_point;
            self.prev_mouse_point = curr;
            if movt_vec.mag() < dead_zone {
                self.deadzone_count += 1;
            }
            if self.deadzone_count > DEADEZONE_COUNT {
                self.prev_dead_point = Some(curr);
                self.deadzone_count = 0;

                let is_dir_up = if diff_vec.x == 0.0
                    && diff_vec.y > 0.0
                {
                    true
                } else {
                    (diff_vec.y / diff_vec.x).abs()
                        > STRAIGHT_RATIO
                        && diff_vec.y > 0.0
                };

                let is_dir_down = if diff_vec.x == 0.0
                    && diff_vec.y < 0.0
                {
                    true
                } else {
                    (diff_vec.y / diff_vec.x).abs()
                        > STRAIGHT_RATIO
                        && diff_vec.y < 0.0
                };

                let is_up =
                    is_dir_up && diff_vec.mag() > dead_zone;

                let is_down = is_dir_down
                    && diff_vec.mag() > dead_zone;

                if is_up {
                    output.up = true;
                    output.down = false;
                }

                if is_down {
                    output.up = false;
                    output.down = true;
                }

                if !is_up && !is_down {
                    if diff_vec.x > dead_zone {
                        output.left = false;
                        output.right = true;
                    }
                    if diff_vec.x < -dead_zone {
                        output.left = true;
                        output.right = false;
                    }
                }
                self.prev_game_input = output.clone();
            }
            output
        } else {
            GameInput::default()
        }
    }
}
