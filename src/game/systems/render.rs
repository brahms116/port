use super::*;

pub fn render_static_system<T: GameApi>(
    world: &mut World,
    api: &T,
    camera_transform: &Transform,
) {
    for (_id, (transform, surface)) in world
        .query_mut::<(&Transform, &RenderStatic)>(
        )
    {
        render(
            transform,
            &surface.0,
            api,
            &camera_transform,
        )
    }
}

pub fn render_system<
    T: 'static + Send + Sync,
    K: GameApi,
>(
    api: &K,
    world: &mut World,
    camera_transform: &Transform,
) {
    for(_id,(state,transform,cb,)) in 
        world.query_mut::<(
            &T,
            &Transform,
            &StateRenderCb<T>
        )>(){
            render(transform,&cb.0(state,api.window_size()),api,camera_transform)
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
            .map(|e| *e + transform.position)
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
