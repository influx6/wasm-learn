use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize, Serialize, Debug)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize, Serialize, Debug)]
struct Cell {
    frame: Rect,
}

#[derive(Deserialize, Serialize, Debug)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

fn _load_image(resource_uri: &str) -> web_sys::HtmlImageElement {
    let image = web_sys::HtmlImageElement::new().unwrap();
    image.set_src(resource_uri);
    image
}

async fn do_load_image(resource_uri: &str) -> Result<web_sys::HtmlImageElement, JsValue> {
    let (sender, receiver) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
    let sender = Rc::new(Mutex::new(Some(sender)));
    let send_error_counter = Rc::clone(&sender);

    let dom_player_image = _load_image(resource_uri);
    let on_load_closure = Closure::once(move || {
        if let Some(sender) = sender.lock().ok().and_then(|mut rx| rx.take())
        {
            sender.send(Ok(()));
        }
    });

    let on_error_closure = Closure::once(move |err| {
        if let Some(send_error_counter) = send_error_counter.lock().ok().and_then(|mut opt| opt.take()) {
            send_error_counter.send(Err(err));
        }
    });

    dom_player_image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
    dom_player_image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));

    on_load_closure.forget();
    on_error_closure.forget();

    match receiver.await {
        Ok(_) => {
            Ok(dom_player_image)
        }
        Err(err) => {
            return Err(JsValue::from_str(format!("Failed to load image: {} due to: {}", resource_uri, err).as_str()));
        }
    }
}

async fn fetch_json<T: DeserializeOwned>(json_path: &str) -> Result<T, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(json_path)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;

    match wasm_bindgen_futures::JsFuture::from(resp.json()?).await {
        Ok(json_object_js_obj) => {
            Ok(serde_wasm_bindgen::from_value::<T>(json_object_js_obj)
                .expect("Expected json object converted correctly from JsValue"))
        }
        Err(err) => {
            Err(err)
        }
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    // #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let _body = document
        .query_selector("body")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlBodyElement>()
        .unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    spawn_local(async move {
        let player_sprite_sheet = fetch_json::<Sheet>("/assets/sprite_sheets/rhb.json").await.expect("Could not fetch rhb.json");
        let player_sheet_image = do_load_image("/assets/sprite_sheets/rhb.png").await.expect("Could not load rhb.png image for player sheet");


        let sprite = player_sprite_sheet.frames.get("Run (1).png").expect("Cell not found");
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&player_sheet_image,
                                                                                          sprite.frame.x.into(),
                                                                                          sprite.frame.y.into(),
                                                                                          sprite.frame.w.into(),
                                                                                          sprite.frame.h.into(),
                                                                                          300.0,
                                                                                          300.0,
                                                                                          sprite.frame.w.into(),
                                                                                          sprite.frame.h.into(),
            ).unwrap();
    });

    Ok(())
}
