use super::*;

pub fn collision_box(
    transform: Transform,
) -> (
    StoryBlocker,
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
        color: RGBA::new(255, 0, 0, 0.0),
    }];

    (
        StoryBlocker(),
        transform,
        RenderStatic(surfaces),
        BoxCollider::new(
            16.0,
            16.0,
            Vec2::default(),
        ),
        CollisionBoxMarker(),
    )
}
