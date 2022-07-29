use super::*;

pub struct Parent {
    pub id: Option<Entity>,
}

impl Parent {
    pub fn new(id: Option<Entity>) -> Self {
        Self { id }
    }
}
