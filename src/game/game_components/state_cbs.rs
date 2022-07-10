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
