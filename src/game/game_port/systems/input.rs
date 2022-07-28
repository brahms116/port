use super::*;

pub fn system_input<T: GameApi>(
    world: &mut World,
    api: &T,
) -> GameInput {
    for (_id, (controller,)) in
        world.query_mut::<(&mut InputController,)>()
    {
        let mouse = api.inputs();
        if mouse.is_down && !controller.is_mouse_down() {
            controller.mouse_down(mouse.pos);
        }
        if !mouse.is_down && controller.is_mouse_down() {
            controller.mouse_up();
        }
        let mouse_output = controller.input(mouse.pos);
        let keyboard_output = api.key_inputs();

        return GameInput {
            up: mouse_output.up || keyboard_output.up,
            down: mouse_output.down || keyboard_output.down,
            left: mouse_output.left || keyboard_output.left,
            right: mouse_output.right
                || keyboard_output.right,
        };
    }
    GameInput::default()
}
