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
            horizontal: if keyboard_output.horizontal == 0.0
            {
                mouse_output.horizontal
            } else {
                keyboard_output.horizontal
            },
            vertical: if keyboard_output.horizontal == 0.0 {
                mouse_output.vertical
            } else {
                keyboard_output.vertical
            },
        };
    }
    GameInput::default()
}
