mod game_api;
mod game_core;

pub use game_api::*;
pub use game_core::*;
use hecs::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Game {
    world: Rc<RefCell<World>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: Rc::new(RefCell::new(World::new())),
        }
    }

    pub fn game_loop<T>(&self) -> impl FnMut(&T)
    where
        T: GameApi,
    {
        let world = Rc::clone(&self.world);
        move |api: &T| {
            let _world = world.borrow_mut();
            api.log(&"hello world")
        }
    }
}
