use super::*;
use hecs::*;

pub fn system_update_frame(n: &mut FrameCount) {
    n.0 += 1;
}

pub fn get_camera_transform(world: &World) -> Transform {
    for (_id, (_cam, pos)) in &mut world.query::<(&Camera, &Transform)>() {
        return pos.clone();
    }
    return Transform::default();
}

pub fn system_render<T: GameApi>(
    transform: &Transform,
    surfaces: &Vec<Surface>,
    api: &T,
    camera_transform: &Transform,
) {
    for surface in surfaces {
        let points: Vec<Vec2> = surface
            .points
            .iter()
            .map(|e| *e + transform.position)
            .collect();
        /* Need to apply rotation */
        let points: Vec<Vec2> = points
            .iter()
            .map(|e| map_vec2(e, &camera_transform.position, api.window_size()))
            .collect();

        let screen_surface = Surface {
            points,
            color: surface.color.clone(),
        };

        api.draw_surface(screen_surface)
    }
}

pub fn system_motion(motion: &mut Motion, transform: Option<&mut Transform>) {
    motion.vel = motion.vel + motion.accel;
    motion.angular_vel += motion.angular_accel;
    //TODO: apply rotation
    if let Some(t) = transform {
        t.position = t.position + motion.vel;
        t.rotation += motion.angular_vel;
    }
}

pub fn collision_player(
    state: &mut PlayerState,
    transform: &mut Transform,
    motion: &mut Motion,
    correction_vec: Vec2,
) {
    let x = 1;
}
