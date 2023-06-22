use wasm_bindgen::prelude::*;

#[macro_use]
mod browser;
mod engine;
mod game;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    // #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    browser::spawn_local_thread(async move {
        let walk_the_dog = game::WalkTheDog::new();
        engine::run_loop(walk_the_dog)
            .await
            .expect("Failed to start game");
    });

    Ok(())
}
