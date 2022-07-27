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

pub fn system_squish_animation_advance<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (squish,)) in
        world.query_mut::<(&mut SquishAnimation,)>()
    {
        if squish.is_active {
            squish.advance_frame();
        }
    }
}

pub fn system_squish_animation<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (squish, height, width, render_offset)) in
        world.query_mut::<(
            &SquishAnimation,
            &mut Height,
            &mut Width,
            Option<&mut RenderOffset>,
        )>()
    {
        if squish.is_active {
            let poll = squish.poll();

            let start_width = if poll < 0.5 {
                squish.config.start_width
            } else {
                squish.config.squish_width
            };

            let start_height = if poll < 0.5 {
                squish.config.start_height
            } else {
                squish.config.squish_height
            };

            let desired_width = if poll < 0.5 {
                squish.config.squish_width
            } else {
                squish.config.finish_width
            };

            let desired_height = if poll < 0.5 {
                squish.config.squish_height
            } else {
                squish.config.finish_height
            };

            let progress = if poll < 0.5 {
                poll * 2.0
            } else {
                2.0 * (poll - 0.5)
            };

            // _api.log(&format!("{}", progress));

            let new_height =
                (desired_height - start_height) * progress
                    + start_height;

            let new_width = (desired_width - start_width)
                * progress
                + start_width;

            match squish.direction {
                SquishDirection::Front
                | SquishDirection::Back => {
                    height.0 = new_height;
                    width.0 = new_width;
                }
                SquishDirection::Left
                | SquishDirection::Right => {
                    width.0 = new_height;
                    height.0 = new_width;
                }
            }

            if squish.config.should_anchor
                && render_offset.is_some()
            {
                let offset = match squish.direction {
                    SquishDirection::Front
                    | SquishDirection::Back => height.0,
                    SquishDirection::Left
                    | SquishDirection::Right => width.0,
                };

                let offset = (squish.config.finish_height
                    - offset)
                    * 0.5;

                let offset = match squish.direction {
                    SquishDirection::Front => {
                        Vec2::new(0.0, offset)
                    }
                    SquishDirection::Back => {
                        Vec2::new(0.0, -offset)
                    }
                    SquishDirection::Left => {
                        Vec2::new(-offset, 0.0)
                    }
                    SquishDirection::Right => {
                        Vec2::new(offset, 0.0)
                    }
                };

                let render_offset = render_offset.unwrap();
                render_offset.0 = offset;
            }
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
