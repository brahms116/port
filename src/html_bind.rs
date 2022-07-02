use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct WindowSize {
    pub w: i32,
    pub h: i32,
}

impl WindowSize {
    pub fn new() -> WindowSize {
        let window = web_sys::window().unwrap();
        let w = window.inner_width().unwrap();
        let h = window.inner_height().unwrap();
        WindowSize {
            w: w.as_f64().unwrap().round() as i32,
            h: h.as_f64().unwrap().round() as i32,
        }
    }
}

pub struct CanvasElement {
    pub canvas: web_sys::HtmlCanvasElement,
    pub ctx: web_sys::CanvasRenderingContext2d,
}

impl CanvasElement {
    pub fn new() -> CanvasElement {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        CanvasElement { canvas, ctx }
    }

    pub fn resize(&self) {
        let size = WindowSize::new();
        self.canvas.set_height(size.h as u32);
        self.canvas.set_width(size.w as u32);
    }
}

pub struct Api {
    canvas: CanvasElement,
}

impl Api {
    pub fn new() -> Api {
        Api {
            canvas: CanvasElement::new(),
        }
    }
    pub fn run<T>(&self, mut f: T)
    where
        T: 'static + FnMut(),
    {
        let window = web_sys::window().unwrap();
        let window2 = web_sys::window().unwrap();

        let h = Rc::new(RefCell::<Option<Closure<dyn FnMut()>>>::new(None));
        let g = Rc::clone(&h);
        let cls = Closure::wrap(Box::new(move || {
            f();
            window
                .request_animation_frame(h.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>);
        *g.borrow_mut() = Some(cls);
        window2
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }
}
