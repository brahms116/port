use super::*;

pub struct CollisionBox {
    pub points: Vec<Vec2>,
}

impl CollisionBox {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}

pub struct StaticCollider();

pub struct DynamicCollider();
