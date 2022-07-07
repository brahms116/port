mod entity;
mod game_api;
mod game_components;
mod game_core;
mod main_loop;
mod setup;
mod systems;

use crate::animation::*;
pub use game_api::*;
use game_components::*;
pub use game_core::*;
use hecs::*;
use main_loop::*;
use setup::*;
use std::cell::RefCell;
use std::rc::Rc;
use systems::*;
pub struct Game {
    world: Rc<RefCell<World>>,
}

impl Game {
    pub fn new() -> Game {
        let mut world = World::new();
        setup(&mut world);
        Game {
            world: Rc::new(RefCell::new(world)),
        }
    }
    pub fn game_loop<T>(&self) -> impl FnMut(&T)
    where
        T: GameApi,
    {
        let world = Rc::clone(&self.world);
        move |api: &T| {
            let mut world = world.borrow_mut();
            main_loop(&mut world, api)
        }
    }
}
