use super::*;

pub struct StateColliderCb<State>(
    pub fn(&State) -> BoxCollider,
);

pub struct RenderStatic(pub Vec<Surface>);

pub struct StateRenderCb<State>(
    pub fn(&State, &WindowSize) -> Vec<Surface>,
);

pub struct StateMotionCb<State>(
    pub  fn(
        &mut State,
        &mut Motion,
        &mut Transform,
    ),
);

pub struct UpdateStateCb<State>(
    pub fn(&mut State),
);

pub struct CollisionCb<T>(
    pub fn(Entity, &mut World, &Vec2),
    pub std::marker::PhantomData<T>,
);

impl<T> CollisionCb<T> {
    pub fn new(
        func: fn(Entity, &mut World, &Vec2),
    ) -> Self {
        Self(func, std::marker::PhantomData::<T>::default())
    }
}
