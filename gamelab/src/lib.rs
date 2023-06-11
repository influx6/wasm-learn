use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[macro_use]
mod browser;
mod engine;

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

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    // #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let context = browser::canvas_context().expect("Canvas.context should be found");

    browser::spawn_local_thread(async move {
        let player_sprite_sheet = browser::fetch_json_as::<Sheet>("/assets/sprite_sheets/rhb.json")
            .await
            .expect("Could not fetch rhb.json");

        let player_sheet_image = engine::do_load_image("/assets/sprite_sheets/rhb.png")
            .await
            .expect("Could not load rhb.png image for player sheet");

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);

            // draw next frame of player sprite
            context.clear_rect(0.0, 0.0, 600.0, 600.0);

            let sprite = player_sprite_sheet
                .frames
                .get(&frame_name)
                .expect("Cell not found");
            context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &player_sheet_image,
                    sprite.frame.x.into(),
                    sprite.frame.y.into(),
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                    300.0,
                    300.0,
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                )
                .unwrap();
        }) as Box<dyn FnMut()>);

        browser::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50,
            )
            .unwrap();

        interval_callback.forget();
    });

    Ok(())
}
