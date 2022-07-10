use super::*;

pub fn get_camera_transform(
    world: &World,
) -> Transform {
    for (_id, (_cam, pos)) in &mut world
        .query::<(&Camera, &Transform)>()
    {
        return pos.clone();
    }
    return Transform::default();
}

pub fn update_game_controller(world: &mut World) {
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        system_update_frame(frame_count);
    }
}

pub fn system_update_frame(n: &mut FrameCount) {
    n.0 += 1;
}
