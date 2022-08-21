use super::*;

pub fn system_player_movt<T: GameApi>(
    world: &mut World,
    _api: &T,
    inputs: &GameInput,
) {
    for (_id, (_player, squish_movt, rotation)) in
        world.query_mut::<(&Player, &mut SquishMovement, &mut Rotation)>()
    {

        if inputs.vertical == 0.0 && 
            (squish_movt.is_moving_forward()||squish_movt.is_moving_back()) {
            squish_movt.stop();
        }
        else if inputs.vertical > 0.0 && !squish_movt.is_moving_forward() {
            squish_movt.forward();
        }
        else if inputs.vertical < 0.0 && !squish_movt.is_moving_back() {
            squish_movt.back();
        }

        if inputs.horizontal == 0.0 && (rotation.is_right() || rotation.is_left()){
            rotation.stop();
        }

        if inputs.vertical > 0.0 {
            if inputs.horizontal>0.0 && !rotation.is_right() {
                rotation.right();
            }

            if inputs.horizontal < 0.0 && !rotation.is_left() {
                rotation.left();
            }
        }
        if inputs.vertical < 0.0 {
            if inputs.horizontal>0.0 && !rotation.is_left() {
                rotation.left();
            }

            if inputs.horizontal < 0.0 && !rotation.is_right() {
                rotation.right();
            }
        }
    }
}
