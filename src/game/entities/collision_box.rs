use super::*;

pub fn collision_box() -> (
    Transform,
    RenderStatic,
    BoxCollider,
    CollisionBoxMarker,
) {
    let surfaces = vec![Surface {
        points: vec![
            Vec2::new(-8.0, 8.0),
            Vec2::new(8.0, 8.0),
            Vec2::new(8.0, -8.0),
            Vec2::new(-8.0, -8.0),
        ],
        color: RGBA::new(255, 0, 0, 1.0),
    }];

    (
        Transform {
            position: Vec2::new(0.0, 200.0),
            rotation: 0.0,
        },
        RenderStatic(surfaces),
        BoxCollider::new(
            16.0,
            16.0,
            Vec2::default(),
        ),
        CollisionBoxMarker(),
    )
}
