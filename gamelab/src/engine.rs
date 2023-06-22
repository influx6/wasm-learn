use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::browser;

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

impl KeyState {
    fn new() -> Self {
        KeyState {
            pressed_keys: HashMap::new(),
        }
    }

    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code.into());
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.x.into(),
            rect.y.into(),
            rect.width.into(),
            rect.height.into(),
        )
    }

    pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                frame.x.into(),
                frame.y.into(),
                frame.width.into(),
                frame.height.into(),
                destination.x.into(),
                destination.y.into(),
                destination.width.into(),
                destination.height.into(),
            )
            .expect("Drawing is throwing exception, unable to recover");
    }
}

#[async_trait(?Send)]
pub trait Game {
    fn update(&mut self, keys: &KeyState);
    fn draw(&self, context: &Renderer);
    async fn initialize(&self) -> Result<Box<dyn Game>>;
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;

type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;

fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);

    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()
        .unwrap()
        .set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    browser::canvas()
        .unwrap()
        .set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));

    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}

fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
            },
        };
    }
}

pub async fn do_load_image(resource_uri: &str) -> Result<web_sys::HtmlImageElement> {
    let dom_player_image = browser::new_image(resource_uri)
        .map_err(|err| anyhow!("could not load image element: {:#?}", err))?;

    let (sender_channel, receiver_channel) = futures::channel::oneshot::channel::<Result<()>>();

    let sender = Rc::new(Mutex::new(Some(sender_channel)));
    let send_error_counter = Rc::clone(&sender);

    let on_load_closure = browser::closure_once(move || {
        if let Some(sender) = sender.lock().ok().and_then(|mut rx| rx.take()) {
            match sender.send(Ok(())) {
                Ok(_) => {}
                Err(err) => {
                    log!("fail to load send success callback: {:#?}", err)
                }
            }
        }
    });

    let on_error_closure: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(send_error_counter) = send_error_counter
            .lock()
            .ok()
            .and_then(|mut opt| opt.take())
        {
            match send_error_counter.send(Err(anyhow!(
                "Failed to deliver error to Oneshot::channel: {:#?}",
                err
            ))) {
                Ok(_) => {}
                Err(err) => {
                    log!("fail to load image: {:#?}", err)
                }
            }
        }
    });

    dom_player_image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
    dom_player_image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));

    receiver_channel
        .await
        .map_err(|err| anyhow!("Failed to load image: {} due to: {:#?}", resource_uri, err))
        .map(|_| Ok(dom_player_image))?
}

pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f32,
}

pub async fn run_loop(game: impl Game + 'static) -> Result<()> {
    let mut keyevent_receiver = prepare_input()?;
    let mut game = game.initialize().await?;
    let mut game_loop = GameLoop {
        last_frame: browser::now()?,
        accumulated_delta: 0.0,
    };

    let renderer = Renderer {
        context: browser::canvas_context()?,
    };

    let f: SharedLoopClosure = Rc::new(RefCell::new(None));
    let g = f.clone();

    // assign browser closure to SharedLoopClosure after definition,
    // that is why we use RefCell to be able hold an option.
    let mut keystate = KeyState::new();
    *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
        process_input(&mut keystate, &mut keyevent_receiver);

        game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;

        while game_loop.accumulated_delta > FRAME_SIZE {
            game.update(&keystate);
            game_loop.accumulated_delta -= FRAME_SIZE;
        }

        game_loop.last_frame = perf;
        game.draw(&renderer);
        browser::request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    browser::request_animation_frame(
        g.borrow()
            .as_ref()
            .ok_or_else(|| anyhow!("GameLoop: loop is None"))?,
    )?;
    Ok(())
}
