use crate::game::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

impl WindowSize {
    pub fn from_dom() -> WindowSize {
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
        let size = WindowSize::from_dom();
        self.canvas.set_height(size.h as u32);
        self.canvas.set_width(size.w as u32);
    }
}

pub struct GameRunner<Api> {
    api: Rc<RefCell<Api>>,
}

impl<Api: 'static> GameRunner<Api> {
    pub fn new(api: Api) -> GameRunner<Api> {
        GameRunner {
            api: Rc::new(RefCell::new(api)),
        }
    }
}

impl GameRunner<HTMLApi> {
    pub fn run<T>(&self, mut f: T)
    where
        T: 'static + FnMut(&HTMLApi),
    {
        let window = web_sys::window().unwrap();
        let window2 = web_sys::window().unwrap();

        let h = Rc::new(RefCell::<
            Option<Closure<dyn FnMut()>>,
        >::new(None));
        let g = Rc::clone(&h);
        let api = Rc::clone(&self.api);
        let cls = Closure::wrap(Box::new(move || {
            let mut api = api.borrow_mut();
            api.update();
            f(&api);
            window
                .request_animation_frame(
                    h.borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap();
        })
            as Box<dyn FnMut()>);
        *g.borrow_mut() = Some(cls);
        window2
            .request_animation_frame(
                g.borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();
    }
}

pub struct HTMLApi {
    canvas: CanvasElement,
    window_size: WindowSize,
    mouse_input: MouseInput,
}

impl HTMLApi {
    pub fn new() -> HTMLApi {
        HTMLApi {
            canvas: CanvasElement::new(),
            window_size: WindowSize::from_dom(),
            mouse_input: MouseInput::default(),
        }
    }

    pub fn update(&mut self) {
        self.canvas.resize();
        self.window_size = WindowSize::from_dom();
    }
}

impl GameApi for HTMLApi {
    fn window_size(&self) -> &WindowSize {
        &self.window_size
    }

    fn draw_surface(&self, surface: Surface) {
        if surface.points.len() > 0 {
            self.canvas.ctx.begin_path();
            self.canvas.ctx.set_fill_style(
                &surface.color.to_string().into(),
            );
            let start = &surface.points[0];
            self.canvas.ctx.move_to(start.x, start.y);
            for (i, v) in surface.points.iter().enumerate()
            {
                if i != 0 {
                    self.canvas.ctx.line_to(v.x, v.y);
                }
                if i == surface.points.len() - 1 {
                    self.canvas
                        .ctx
                        .line_to(start.x, start.y);
                    self.canvas.ctx.fill();
                }
            }
        }
    }

    fn set_element_position(
        &self,
        _point: Vec2,
        _id: &str,
    ) {
        let style = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .style();

        style
            .set_property("top", &format!("{}px", _point.y))
            .unwrap();
        style
            .set_property(
                "left",
                &format!("{}px", _point.x),
            )
            .unwrap();
    }

    fn resize_element(&self, _size: u32, _id: &str) {
        let style = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .style();
        style
            .set_property(
                "font-size",
                &format!("{}px", _size),
            )
            .unwrap();
    }

    fn set_element_opacity(
        &self,
        _opacity: u32,
        _id: &str,
    ) {
        todo!()
    }

    fn inputs(&self) -> &MouseInput {
        &self.mouse_input
    }

    fn log(&self, msg: &str) {
        web_sys::console::log_1(&msg.into())
    }
}
