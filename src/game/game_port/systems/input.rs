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
        return controller.input(mouse.pos);
    }
    GameInput::default()
}
