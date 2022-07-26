use super::*;

pub fn system_rectangle_render<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for(_id,(_rect, width,height, render_surface)) in world.query_mut::<(&Rectangle, &Width, &Height,&mut Render)>(){
        let (vec1,vec2,vec3,vec4) = Rect{
            width:width.0,
            height:height.0,
            transform:Transform::default()
        }.corners();
        let points = vec![vec1,vec2,vec3,vec4];
        let surface = render_surface.0.get_mut(0);
        if let Some(surface) = surface {
            surface.points = points
        }
    }
}
