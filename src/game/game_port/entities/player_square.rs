use super::*;

pub fn create_player_square(
    transform: Transform,
) -> (Transform, Render) {
    let (vec1, vec2, vec3, vec4) = Rect::two_point_flat(
        Vec2::default(),
        Vec2::new(16.0, 16.0),
    )
    .corners();

    let points = vec![vec1, vec2, vec3, vec4];

    let surface = Surface {
        points,
        color: RGBA::new(0, 0, 0, 1.0),
    };

    (transform, Render(vec![surface]))
}
