use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::browser;

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
    fn update(&mut self);
    fn draw(&self, context: &Renderer);
    async fn initialize(&self) -> Result<Box<dyn Game>>;
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;

type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;

pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f32,
}

pub async fn run_loop(game: impl Game + 'static) -> Result<()> {
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
    *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
        game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;

        while game_loop.accumulated_delta > FRAME_SIZE {
            game.update();
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
