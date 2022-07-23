use super::*;
type S = PlayerStateKind;
type D = PlayerDirection;

pub fn player_square(
    transform: Transform,
    motion: Motion,
    state: PlayerState,
) -> (
    Player,
    Transform,
    Motion,
    PlayerState,
    StateRenderCb<PlayerState>,
    StateMotionCb<PlayerState>,
    StateColliderCb<PlayerState>,
    UpdateStateCb<PlayerState>,
    CollisionCb<CollisionBoxMarker, PlayerState>,
) {
    (
        Player(),
        transform,
        motion,
        state,
        StateRenderCb(render_cb),
        StateMotionCb(motion_cb),
        StateColliderCb(collider_cb),
        UpdateStateCb(update_state_cb),
        CollisionCb::new(collision_cb),
    )
}

fn collision_cb(
    id: Entity,
    world: &World,
    correction_vec: &Vec2,
) {
    let mut p_transform =
        world.get_mut::<Transform>(id).unwrap();
    let mut p_motion =
        world.get_mut::<Motion>(id).unwrap();
    let mut p_state =
        world.get_mut::<PlayerState>(id).unwrap();
    collision_player(
        &mut *p_state,
        &mut *p_transform,
        &mut *p_motion,
        correction_vec,
    )
}
fn collision_player(
    state: &mut PlayerState,
    transform: &mut Transform,
    motion: &mut Motion,
    correction_vec: &Vec2,
) {
    let relative_correction = correction_vec
        .rotate_deg(-transform.rotation);
    let direction = if relative_correction.y < 0.0
    {
        D::Front
    } else {
        D::Back
    };

    *state = PlayerState::post_motion(direction);
    motion.vel = Vec2::default();
    motion.accel = Vec2::default();
    transform.position += *correction_vec;
}

fn render_cb(
    state: &PlayerState,
    _size: &WindowSize,
) -> Vec<Surface> {
    let color = RGBA::new(43, 43, 43, 1.0);

    let still_height = 16.0;
    let still_width = 16.0;

    let x = still_width / 2.0;
    let y = still_height / 2.0;

    if let S::Still = state.state {
        return vec![Surface {
            points: vec![
                Vec2::new(-x, -y),
                Vec2::new(-x, y),
                Vec2::new(x, y),
                Vec2::new(x, -y),
            ],
            color,
        }];
    }

    let squish_width = 28.0;
    let squish_height = 2.0;

    let moving_width = 8.0;
    let moving_height = 20.0;

    let squish = |factor: f64,
                  start_width: f64,
                  start_height: f64,
                  end_width: f64,
                  end_height: f64|
     -> (f64, f64) {
        if factor < 0.5 {
            return (
                start_width
                    + (squish_width
                        - start_width)
                        * factor
                        * 2.0,
                start_height
                    - (start_height
                        - squish_height)
                        * factor
                        * 2.0,
            );
        } else {
            return (
                end_width
                    + (squish_width - end_width)
                        * (1.0 - factor)
                        * 2.0,
                end_height
                    - (end_height
                        - squish_height)
                        * (1.0 - factor)
                        * 2.0,
            );
        }
    };

    let mut results: Vec<Vec2> = vec![];
    match &state.state {
        S::PostMotion(n) => {
            let val = n.0.poll();
            let is_check = n.0.checkpoint() > 0;
            let offset =
                if is_check { 0.0 } else { 2.0 };
            let (width, height) = squish(
                val,
                moving_width,
                moving_height,
                still_width,
                still_height,
            );
            let mut factor = 1.0;
            if let D::Back = n.1 {
                factor = -1.0;
            }
            results = vec![
                Vec2::new(
                    -width / 2.0,
                    (still_height / 2.0 + offset)
                        * factor,
                ),
                Vec2::new(
                    width / 2.0,
                    (still_height / 2.0 + offset)
                        * factor,
                ),
                Vec2::new(
                    width / 2.0,
                    (still_height / 2.0 + offset
                        - height)
                        * factor,
                ),
                Vec2::new(
                    -width / 2.0,
                    (still_height / 2.0 + offset
                        - height)
                        * factor,
                ),
            ];
        }
        S::Motion(n) | S::Jump(n) => {
            let val = n.poll();
            let is_moved = n.checkpoint() == 1;
            let offset =
                if is_moved { 2.0 } else { 0.0 };
            let (width, height) = squish(
                val,
                still_width,
                still_height,
                moving_width,
                moving_height,
            );
            results = vec![
                Vec2::new(
                    -width / 2.0,
                    -still_height / 2.0 - offset,
                ),
                Vec2::new(
                    width / 2.0,
                    -still_height / 2.0 - offset,
                ),
                Vec2::new(
                    width / 2.0,
                    -still_height / 2.0 - offset
                        + height,
                ),
                Vec2::new(
                    -width / 2.0,
                    -still_height / 2.0 - offset
                        + height,
                ),
            ]
        }
        _ => {}
    }

    vec![
        Surface {
            points: results,
            color,
        },
        // Surface {
        //     points: vec![
        //         Vec2::new(-4.0, -4.0),
        //         Vec2::new(0.0, 4.0),
        //         Vec2::new(4.0, -4.0),
        //     ],
        //     color: RGBA::new(0, 255, 0, 1.0),
        // },
    ]
}

fn motion_cb(
    state: &mut PlayerState,
    motion: &mut Motion,
    transform: &mut Transform,
) {
    let offset = 2.0;
    match &mut state.state {
        S::PostMotion(n) => {
            if n.0.checkpoint() == 0 {
                n.0.advance_checkpoint();
                if let D::Front = n.1 {
                    transform.position = transform
                        .position
                        + Vec2::new(0.0, offset)
                            .rotate_deg(
                                transform
                                    .rotation,
                            );
                } else {
                    transform.position = transform
                        .position
                        + Vec2::new(0.0, -offset)
                            .rotate_deg(
                                transform
                                    .rotation,
                            )
                }
            }
        }
        S::Motion(n) => {
            if n.checkpoint() == 0 {
                n.advance_checkpoint();
                transform.position = transform
                    .position
                    + Vec2::new(0.0, offset)
                        .rotate_deg(
                            transform.rotation,
                        )
            } else if n.checkpoint() == 2 {
                n.advance_checkpoint();
                if motion.vel.y
                    < state.config.max_travel_vel
                {
                    motion.accel = motion.accel
                        + Vec2::new(
                            0.0,
                            state
                                .config
                                .travel_accel,
                        )
                }
            } else if n.checkpoint() == 3 {
                if motion.vel.y
                    > state.config.max_travel_vel
                {
                    motion.vel.y = state
                        .config
                        .max_travel_vel;

                    motion.accel -= Vec2::new(
                        0.0,
                        state.config.travel_accel,
                    )
                }
            }
        }
        S::Jump(n) => {
            if n.checkpoint() == 0 {
                n.advance_checkpoint();
                transform.position = transform
                    .position
                    + Vec2::new(0.0, offset)
                        .rotate_deg(
                            transform.rotation,
                        )
            } else if n.checkpoint() == 2 {
                n.advance_checkpoint();
                motion.vel = Vec2::new(
                    0.0,
                    state.config.max_travel_vel,
                );
                motion.accel =
                    Vec2::new(0.0, -0.5);
            }
        }
        _ => {}
    }
}

fn collider_cb(
    state: &PlayerState,
) -> BoxCollider {
    let normal = BoxCollider::new(
        16.0,
        16.0,
        Vec2::default(),
    );

    let moving = BoxCollider::new(
        8.0,
        20.0,
        Vec2::default(),
    );

    match &state.state {
        S::Still | S::PostMotion(_) => normal,
        S::Jump(n) | S::Motion(n) => {
            if n.is_complete() {
                return moving;
            }
            return normal;
        }
    }
}

fn update_state_cb(state: &mut PlayerState) {
    match &mut state.state {
        S::PostMotion(n) => {
            if n.0.is_complete() {
                *state = PlayerState::still();
            } else {
                n.0.advance_frame();
            }
        }
        S::Motion(n) | S::Jump(n) => {
            n.advance_frame();
            if n.poll() > 0.5
                && n.checkpoint() < 2
            {
                n.advance_checkpoint();
            }
        }
        _ => {}
    }
}
