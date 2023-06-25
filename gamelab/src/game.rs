use anyhow::{anyhow, Result};
use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::HtmlImageElement;

use crate::avatar;
use crate::browser;
use crate::engine;
use crate::models;

pub enum WalkTheDog {
    Loading,
    Loaded(avatar::RedHatBoy),
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog::Loading
    }
}

#[async_trait(?Send)]
impl engine::Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        match self {
            WalkTheDog::Loading => {
                let player_sprite_sheet =
                    browser::fetch_json_as::<models::Sheet>("/assets/sprite_sheets/rhb.json")
                        .await?;

                let player_sheet_image =
                    engine::do_load_image("/assets/sprite_sheets/rhb.png").await?;

                let rhb = avatar::RedHatBoy::new(player_sprite_sheet, player_sheet_image);

                Ok(Box::new(WalkTheDog::Loaded(rhb)))
            }
            WalkTheDog::Loaded(_) => Err(anyhow!("Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        if let WalkTheDog::Loaded(rhb) = self {
            if keystate.is_pressed("Space") {
                rhb.jump();
            }
            if keystate.is_pressed("ArrowDown") {
                rhb.slide();
            }
            if keystate.is_pressed("ArrowUp") {
                rhb.run_up();
            }
            if keystate.is_pressed("ArrowRight") {
                rhb.run_right();
            }
            if keystate.is_pressed("ArrowLeft") {
                rhb.run_left();
            }
            rhb.update();
        }
    }

    fn draw(&self, renderer: &engine::Renderer) {
        renderer.clear(&models::Rect {
            x: 0.0,
            y: 0.0,
            width: 1200.0,
            height: 1200.0,
        });

        if let WalkTheDog::Loaded(rhb) = self {
            rhb.draw(renderer);
        }
    }
}
