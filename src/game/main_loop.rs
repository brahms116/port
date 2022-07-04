use super::game_api::*;
use super::game_components::*;
use super::systems::*;
use hecs::*;

pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    /* Game Controller */
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        api.log(&format!("{}", frame_count.0));
        update_frame(frame_count);
    }
}
