use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use std::error::Error;
use std::future::Future;
use wasm_bindgen::closure::WasmClosureFnOnce;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use rand::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Response, Window};

macro_rules! log {
    ( $($t:tt)* ) => {
        web_sys::console::log_1(&format!( $($t)*).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document Found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No canvas element found with giving id"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn canvas_context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d") // returns a Result<Option<Result<JsObject, Err>, Err>, hence the map_err
        .map_err(|js_value| anyhow!("Errro getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("Canvas failed to return 2d context from Canvas.get_context(2d)"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|element| {
            anyhow!(
                "Error converting {:#?} to CanvasRenderingContext2d",
                element
            )
        })
}

pub fn spawn_local_thread<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
    wasm_bindgen_futures::JsFuture::from(window()?.fetch_with_str(resource))
        .await
        .map_err(|err| anyhow!("errpr fetching {:#?}", err))
}

pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
    let resp_value = fetch_with_str(json_path).await?;
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|element| anyhow!("Error converting {:#?} to Response", element))?;
    wasm_bindgen_futures::JsFuture::from(
        resp.json()
            .map_err(|err| anyhow!("Could not get Json from response {:#?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("error fetching JSON {:#?}", err))
}

pub async fn fetch_json_as<T: DeserializeOwned>(json_path: &str) -> Result<T> {
    fetch_json(json_path)
        .await
        .map_err(|err| anyhow!("Could not fetch data from endpoint: {:#?}", err))
        .map(|json_object| {
            serde_wasm_bindgen::from_value::<T>(json_object).map_err(|err| {
                anyhow!(
                    "Expected json object converted correctly from JsValue: {:#?}",
                    err
                )
            })
        })?
        .map_err(|err| anyhow!("Failed to transform response: {:#?}", err))
}

pub fn new_image(resource_uri: &str) -> Result<web_sys::HtmlImageElement> {
    let image = web_sys::HtmlImageElement::new()
        .map_err(|err| anyhow!("Could not create HtmlImageElement: {:#?}", err))?;
    image.set_src(resource_uri);
    Ok(image)
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}
