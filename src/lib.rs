mod game;
mod html_bind;

use wasm_bindgen::prelude::*;

use game::Game;
use html_bind::*;

#[wasm_bindgen(start)]
pub fn main() {
    let api = HTMLApi::new();
    let runner = GameRunner::new(api);
    let game = Game::new();
    let f: Box<dyn FnMut(&HTMLApi)> =
        Box::new(game.game_loop());
    runner.run(f)
}
