use super::*;

const DEADEZONE_COUNT: u32 = 5;

// const STRAIGHT_RATIO: f64 = 1.0;

pub struct Player();

pub struct InputController {
    prev_dead_point: Option<Vec2>,
    prev_mouse_point: Vec2,
    prev_game_input: GameInput,
    deadzone_count: u32,
    frame_count: u32,
    net_vec: Vec2,
    is_travelling: bool,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            prev_dead_point: None,
            prev_mouse_point: Vec2::default(),
            prev_game_input: GameInput::default(),
            deadzone_count: 0,
            frame_count: 0,
            net_vec: Vec2::default(),
            is_travelling: false,
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
        self.frame_count = 0;
        self.net_vec = Vec2::default();
        self.is_travelling = false;
        self.prev_game_input = GameInput::default();
    }

    pub fn input(&mut self, curr: Vec2) -> GameInput {
        let dead_zone = 10.0;

        if let Some(pt) = self.prev_dead_point {
            let diff_vec = curr - pt;
            let movt_vec = curr - self.prev_mouse_point;
            self.prev_mouse_point = curr;

            let mut start_travel_vec: Option<Vec2> = None;

            /* Handle dead_zones */
            if self.deadzone_count >= DEADEZONE_COUNT {
                self.deadzone_count = 0;
                start_travel_vec = Some(curr - pt);
                self.prev_dead_point = Some(curr);
            } else if movt_vec.mag() < dead_zone {
                self.deadzone_count += 1;
            } else {
                self.deadzone_count = 0;
            }

            if self.is_travelling {
                self.frame_count += 1;
                self.net_vec += diff_vec;
                if self.frame_count == 5 {
                    self.frame_count = 0;
                    if self.net_vec.mag() > 5.0 {
                        if self.net_vec.y < -30.0
                            && (self.net_vec.y
                                / self.net_vec.x)
                                .abs()
                                > 3.5
                        {
                            self.prev_game_input.up = false;
                            self.prev_game_input.left =
                                false;
                            self.prev_game_input.right =
                                false;
                            self.prev_game_input.down =
                                true;
                        } else {
                            if self.net_vec.x > 0.0 {
                                let was_left = self
                                    .prev_game_input
                                    .left;
                                self.prev_game_input.left =
                                    false;
                                self.prev_game_input
                                    .right = self.net_vec.x
                                    > 50.0
                                    || !was_left;
                            } else {
                                let was_right = self
                                    .prev_game_input
                                    .right;
                                self.prev_game_input
                                    .right = false;
                                self.prev_game_input.left =
                                    self.net_vec.x < -50.0
                                        || !was_right;
                            }
                        }
                    }
                    self.net_vec = Vec2::default();
                }
            } else if let Some(travel_vec) =
                start_travel_vec
            {
                if travel_vec.mag() > dead_zone {
                    let ratio =
                        (travel_vec.y / travel_vec.x).abs();
                    if ratio < 0.5 {
                        self.prev_game_input.left =
                            travel_vec.x < 0.0;
                        self.prev_game_input.right =
                            travel_vec.x > 0.0;
                    } else {
                        self.is_travelling = true;
                        self.prev_game_input.up =
                            travel_vec.y > 0.0;
                        self.prev_game_input.down =
                            travel_vec.y < 0.0;
                    }
                }
            }

            self.prev_game_input.clone()
        } else {
            GameInput::default()
        }
    }
}
