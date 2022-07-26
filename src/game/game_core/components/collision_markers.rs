pub trait StaticCollisionMarker {}

pub struct CollisionBoxMarker();

impl StaticCollisionMarker
    for CollisionBoxMarker
{
}
