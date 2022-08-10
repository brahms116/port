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
