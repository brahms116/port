use super::*;

pub fn system_fade_animation_advance<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (fade,)) in
        world.query_mut::<(&mut FadeAnimation,)>()
    {
        if fade.is_active {
            fade.advance_frame();
        }
    }
}

pub fn system_fade_animation<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (fade, opacity)) in
        world.query_mut::<(&FadeAnimation, &mut Opacity)>()
    {
        if fade.is_active {
            let value = fade.poll();
            match fade.animation_type {
                FadeAnimationType::In => {
                    opacity.0 = value;
                }
                FadeAnimationType::Out => {
                    opacity.0 = 1.0 - value;
                }
            }
        }
    }
}
