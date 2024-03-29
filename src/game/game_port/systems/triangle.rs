use super::*;

pub fn system_triangle_render<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for(_id,(_tri, width,height, render_surface)) in world.query_mut::<(&Triangle, &Width, &Height,&mut Render)>(){

        let points = vec![
            Vec2::new(-width.0/2.0,-height.0/2.0),
            Vec2::new(0.0,height.0/2.0),
            Vec2::new(width.0/2.0,-height.0/2.0),
        ];
        let surface = render_surface.0.get_mut(0);
        if let Some(surface) = surface {
            surface.points = points
        }
    }
}

pub fn system_triangle_collision_box<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (
        _id,
        (_tri, width, height, collision_box, squish),
    ) in world.query_mut::<(
        &Triangle,
        &Width,
        &Height,
        &mut CollisionBox,
        Option<&SquishAnimation>,
    )>() {
        if let Some(animation) = squish {
            let width = animation.config.finish_width;
            let height = animation.config.finish_height;
            let points = vec![
                Vec2::new(-width / 2.0, -height / 2.0),
                Vec2::new(0.0, height / 2.0),
                Vec2::new(width / 2.0, -height / 2.0),
            ];
            collision_box.points = points;
        } else {
            let points = vec![
                Vec2::new(-width.0 / 2.0, -height.0 / 2.0),
                Vec2::new(0.0, height.0 / 2.0),
                Vec2::new(width.0 / 2.0, -height.0 / 2.0),
            ];
            collision_box.points = points;
        }
    }
}
