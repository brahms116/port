mod html_bind;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

use html_bind::*;

struct Demo {
    count: i32,
}

#[wasm_bindgen(start)]
pub fn main() {
    let count = Rc::new(RefCell::new(Demo { count: 1 }));
    let count_2 = Rc::clone(&count);
    let cls = move || {
        let mut count = count_2.borrow_mut();
        count.count += 1;
        console::log_1(&count.count.into());
    };
    let api = Api::new();
    api.run(cls);
}
