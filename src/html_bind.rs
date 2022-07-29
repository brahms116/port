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
        let my_api = Rc::new(RefCell::new(api));

        GameRunner { api: my_api }
    }
}

impl GameRunner<HTMLApi> {
    pub fn run<T>(&self, mut f: T)
    where
        T: 'static + FnMut(&HTMLApi),
    {
        let window = web_sys::window().unwrap();
        let window2 = web_sys::window().unwrap();

        {
            let api = self.api.clone();
            let window = web_sys::window().unwrap();
            let closure = Closure::<dyn FnMut(_)>::new(
                move |event: web_sys::PointerEvent| {
                    let height = window
                        .inner_height()
                        .unwrap()
                        .as_f64()
                        .unwrap();
                    let mut api = api.borrow_mut();
                    let x = event.offset_x() as f64;
                    let y =
                        height - event.offset_y() as f64;
                    let vec = Vec2::new(x, y);
                    api.mouse_input.pos = vec;
                },
            );
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback(
                    "pointermove",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();
        }

        {
            let window = web_sys::window().unwrap();
            let api = self.api.clone();
            let closure = Closure::<dyn FnMut(_)>::new(
                move |event: web_sys::PointerEvent| {
                    let mut api = api.borrow_mut();
                    api.mouse_input.is_down = true;
                    let height = window
                        .inner_height()
                        .unwrap()
                        .as_f64()
                        .unwrap();
                    let x = event.offset_x() as f64;
                    let y =
                        height - event.offset_y() as f64;
                    let vec = Vec2::new(x, y);
                    api.mouse_input.pos = vec;
                },
            );
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback(
                    "pointerdown",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();
        }

        {
            let api = self.api.clone();
            let closure = Closure::<dyn FnMut(_)>::new(
                move |_event: web_sys::PointerEvent| {
                    let mut api = api.borrow_mut();
                    api.mouse_input.is_down = false;
                },
            );
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback(
                    "pointerup",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();
        }

        {
            let api = self.api.clone();
            let closure = Closure::<dyn FnMut(_)>::new(
                move |event: web_sys::KeyboardEvent| {
                    let mut api = api.borrow_mut();
                    let key = event.key();
                    if key == "w" || key == "ArrowUp" {
                        api.keyboard_input.down = false;
                        api.keyboard_input.up = true;
                    } else if key == "a"
                        || key == "ArrowLeft"
                    {
                        api.keyboard_input.right = false;
                        api.keyboard_input.left = true;
                    } else if key == "s"
                        || key == "ArrowDown"
                    {
                        api.keyboard_input.up = false;
                        api.keyboard_input.down = true;
                    } else if key == "d"
                        || key == "ArrowRight"
                    {
                        api.keyboard_input.left = false;
                        api.keyboard_input.right = true;
                    }
                },
            );
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback(
                    "keydown",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();
        }

        {
            let api = self.api.clone();
            let closure = Closure::<dyn FnMut(_)>::new(
                move |event: web_sys::KeyboardEvent| {
                    let mut api = api.borrow_mut();
                    let key = event.key();
                    if key == "w" || key == "ArrowUp" {
                        api.keyboard_input.up = false;
                    } else if key == "a"
                        || key == "ArrowLeft"
                    {
                        api.keyboard_input.left = false;
                    } else if key == "s"
                        || key == "ArrowDown"
                    {
                        api.keyboard_input.down = false;
                    } else if key == "d"
                        || key == "ArrowRight"
                    {
                        api.keyboard_input.right = false;
                    }
                },
            );
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback(
                    "keyup",
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();
        }

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
    keyboard_input: GameInput,
}

impl HTMLApi {
    pub fn new() -> HTMLApi {
        HTMLApi {
            canvas: CanvasElement::new(),
            window_size: WindowSize::from_dom(),
            mouse_input: MouseInput::default(),
            keyboard_input: GameInput::default(),
        }
    }

    pub fn update(&mut self) {
        self.canvas.resize();
        self.window_size = WindowSize::from_dom();
        self.canvas.ctx.set_fill_style(
            &RGBA::new(16, 16, 16, 1.0).to_string().into(),
        );
        self.canvas.ctx.fill_rect(
            0.0,
            0.0,
            self.window_size.w as f64,
            self.window_size.h as f64,
        )
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
                &format!("{}px", _opacity),
            )
            .unwrap();
    }

    fn inputs(&self) -> &MouseInput {
        &self.mouse_input
    }

    fn key_inputs(&self) -> &GameInput {
        &self.keyboard_input
    }

    fn log(&self, msg: &str) {
        web_sys::console::log_1(&msg.into())
    }
}
