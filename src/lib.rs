use hecs::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() {
    let mut world = World::new();
    // Nearly any type can be used as a component with zero boilerplate
    let a = world.spawn((123, true, "abc"));
    let b = world.spawn((42, false));
    // Systems can be simple for loops
    for (id, (number, &flag)) in world.query_mut::<(&mut i32, &bool)>() {
        if flag {
            *number *= 2;
        }
    }
    console::log_1(&"hello world".into())
}
