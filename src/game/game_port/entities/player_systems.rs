use super::*;

pub fn system_player_movt<T: GameApi>(
    world: &mut World,
    _api: &T,
    inputs: &GameInput,
) {
    for (_id, (_player, squish_movt, rotation)) in
        world.query_mut::<(&Player, &mut SquishMovement, &mut Rotation)>()
    {
        if inputs.up && !squish_movt.is_moving_forward() {
            squish_movt.forward();
        }
        if !inputs.up && squish_movt.is_moving_forward() {
            squish_movt.stop();
            rotation.stop();
        }

        if inputs.down && !squish_movt.is_moving_back() {
            squish_movt.back();
        }
        if !inputs.down && squish_movt.is_moving_back() {
            squish_movt.stop();
            rotation.stop();
        }

        if inputs.up && inputs.left && !rotation.is_left() {
            rotation.left();
        }
        if inputs.up && !inputs.left && rotation.is_left() {
            rotation.stop();
        }

        if inputs.up && inputs.right && !rotation.is_right() {
            rotation.right();
        }
        if inputs.up && !inputs.right && rotation.is_right() {
            rotation.stop();
        }

        if inputs.down && inputs.left && !rotation.is_right() {
            rotation.right();
        }
        if inputs.down && !inputs.left && rotation.is_right() {
            rotation.stop();
        }

        if inputs.down && inputs.right && !rotation.is_left() {
            rotation.left();
        }
        if inputs.down && !inputs.right && rotation.is_left() {
            rotation.stop();
        }
    }
}
