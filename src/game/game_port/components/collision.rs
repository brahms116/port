use super::*;

#[derive(Debug)]
pub struct CollisionBox {
    pub points: Vec<Vec2>,
}

impl CollisionBox {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}

#[derive(Debug)]
pub struct StaticCollider();

pub struct DynamicCollider();
