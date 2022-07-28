use super::*;

pub fn system_player_movt<T: GameApi>(
    world: &mut World,
    _api: &T,
    inputs: &GameInput,
) {
    for (_id, (_player, squish_movt)) in
        world.query_mut::<(&Player, &mut SquishMovement)>()
    {
        if inputs.up && !squish_movt.is_moving() {
            squish_movt.start_movt();
        }
        if !inputs.up && squish_movt.is_moving() {
            squish_movt.stop();
        }
    }
}
