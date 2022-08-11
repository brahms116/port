use super::*;

pub fn system_render_ui<T: GameApi>(
    world: &World,
    api: &T,
) {
    let camera_transform = get_camera_transform(world, api)
        .unwrap_or_default();

    for (_id, (transform, ui)) in
        &mut world.query::<(&Transform, &UI)>()
    {
        let resolved = (transform.position
            - camera_transform.position)
            .rotate_deg(-camera_transform.rotation)
            + camera_transform.position;

        let screen_point = map_vec2(
            &resolved,
            &camera_transform.position,
            api.window_size(),
        );

        api.set_element_position(screen_point, &ui.html_id);
        api.set_element_rotation(
            transform.rotation - camera_transform.rotation,
            &ui.html_id,
        );
    }
}
pub fn system_render<T: GameApi>(world: &World, api: &T) {
    let camera_transfrom = get_camera_transform(world, api)
        .unwrap_or_default();
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
            let rotated =
                offset.0.rotate_deg(transform.rotation);
            transform.position += rotated
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
            .map(|e| {
                let resolved = e.apply(transform);
                let resolved = (resolved
                    - camera_transform.position)
                    .rotate_deg(-camera_transform.rotation)
                    + camera_transform.position;
                resolved
            })
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
