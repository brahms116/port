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

fn get_player_entity(
    world: &World,
) -> Option<Entity> {
    world
        .query::<(&PlayerState,)>()
        .into_iter()
        .next()
        .map(|e| e.0)
}

pub fn controller(
    world: &mut World,
    _api: &impl GameApi,
) {
    type P = Progression;

    let controller: Entity = world
        .query_mut::<(&mut Controller,)>()
        .into_iter()
        .next()
        .map(|e| e.0)
        .expect("controller expected");

    let mut controller = world
        .get_mut::<Controller>(controller)
        .unwrap();

    controller.timer.advance_frame();
    let progress = controller.progress.clone();

    match progress {
        _ => {}
    }

    if let P::BounceTimerStart = progress {
        let player = get_player_entity(world)
            .expect("player expected");
        let player = world
            .get_mut::<PlayerState>(player)
            .expect("player state expected");

        if let PlayerStateKind::Still =
            player.state
        {
            controller.timer =
                LinearAnimation::new(50);
            controller.progress =
                Progression::BounceTimerWait;
        }
    } else if let P::BounceTimerWait = progress {
        let player = get_player_entity(world)
            .expect("player expected");
        let mut player = world
            .get_mut::<PlayerState>(player)
            .expect("player state expected");

        if let PlayerStateKind::Still =
            player.state
        {
            if controller.timer.is_complete() {
                *player = PlayerState::jump();
                controller.progress =
                    Progression::BounceTimerStart;
            }
        }
    }
}
