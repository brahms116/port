mod game;
mod html_bind;

use wasm_bindgen::prelude::*;

use game::Game;
use html_bind::*;

struct Demo {
    count: i32,
}

#[wasm_bindgen(start)]
pub fn main() {
    let api = HTMLApi::new();
    let runner = GameRunner::new(api);
    let game = Game::new();
    let f: Box<dyn FnMut(&HTMLApi)> = Box::new(game.game_loop());
    runner.run(f)

    //let count = Rc::new(RefCell::new(Demo { count: 1 }));
    //let count_2 = Rc::clone(&count);
    //let cls = move || {
    //    let mut count = count_2.borrow_mut();
    //    count.count += 1;
    //    console::log_1(&count.count.into());
    //};
    //let runner = GameRunner::new();
    ////runner.run(cls);
}
