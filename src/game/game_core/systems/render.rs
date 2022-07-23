use super::*;

pub fn system_render<T: GameApi>(world: &World, api: &T) {
    let camera_transfrom =
        get_camera_transform(world).unwrap_or_default();
    for (
        _id,
        (transform, render_surface, opacity, offset),
    ) in &mut world.query::<(
        &Transform,
        &Render,
        Option<&Opacity>,
        Option<&RenderOffset>,
    )>() {
        let mut surfaces = render_surface.0.clone();
        if let Some(o) = opacity {
            for surface in &mut surfaces {
                surface.color.a *= o.0;
            }
        }

        let mut transform = transform.clone();

        if let Some(offset) = offset {
            transform.position += offset.0
        }

        render(
            &transform,
            &surfaces,
            api,
            &camera_transfrom,
        );
    }
}

pub fn render<T: GameApi>(
    transform: &Transform,
    surfaces: &Vec<Surface>,
    api: &T,
    camera_transform: &Transform,
) {
    for surface in surfaces {
        let points: Vec<Vec2> = surface
            .points
            .iter()
            .map(|e| e.apply(transform))
            .collect();
        /* Need to apply rotation */
        let points: Vec<Vec2> = points
            .iter()
            .map(|e| {
                map_vec2(
                    e,
                    &camera_transform.position,
                    api.window_size(),
                )
            })
            .collect();

        let screen_surface = Surface {
            points,
            color: surface.color.clone(),
        };

        api.draw_surface(screen_surface)
    }
}
