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

pub struct CollisionCb<
    T: StaticCollisionMarker,
    K,
>(
    pub fn(Entity, &mut World, &Vec2),
    std::marker::PhantomData<T>,
    std::marker::PhantomData<K>,
);

impl<T: StaticCollisionMarker, K>
    CollisionCb<T, K>
{
    pub fn new(
        func: fn(Entity, &mut World, &Vec2),
    ) -> Self {
        Self(func, std::marker::PhantomData::<T>::default(),
        std::marker::PhantomData::<K>::default())
    }
}
