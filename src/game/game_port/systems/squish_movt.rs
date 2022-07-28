use super::*;

type State = SquishMovementState;

pub fn system_squish_movt<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (sm, components)) in world.query_mut::<(
        &mut SquishMovement,
        Option<(
            &mut SquishAnimation,
            &mut Movement,
            &mut Transform,
        )>,
    )>() {
        if let Some((squish, movt, transform)) = components
        {
            match sm.state {
                State::Idle => {}
                State::WaitingAnimation => {
                    squish.config =
                        player_squish_setting_start();
                    squish.back_mut();

                    let offset =
                        squish.config.finish_height
                            - squish.config.start_height;
                    let offset =
                        Vec2::new(0.0, offset * 0.5)
                            .rotate_deg(transform.rotation);
                    transform.position += offset;

                    sm.state = State::StartedAnimation;
                }
                State::StartedAnimation => {
                    let poll = squish.poll();
                    if poll > 0.2 {
                        movt.forward_mut();
                        sm.state = State::ForceApplied
                    }
                }
                State::ForceApplied => {}
                State::WaitingStopAnimation => {
                    squish.config =
                        player_squish_setting_stop();
                    squish.front_mut();

                    let offset =
                        squish.config.finish_height
                            - squish.config.start_height;
                    let offset =
                        Vec2::new(0.0, offset * 0.5)
                            .rotate_deg(transform.rotation);
                    transform.position += offset;
                    movt.stop_mut();
                    sm.state = State::StartedStopAnimation
                }
                State::StartedStopAnimation => {}
            }
        }
    }
}
