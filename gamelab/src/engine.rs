use std::rc::Rc;
use std::sync::Mutex;

use anyhow::{anyhow, Result};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
