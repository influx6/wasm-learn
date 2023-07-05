use anyhow::{anyhow, Result};
use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::HtmlImageElement;

use crate::avatar;
use crate::browser;
use crate::engine;
use crate::models;

pub struct Walk {
    boy: avatar::RedHatBoy,
    stone: engine::Image,
    background: engine::Image,
}

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
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
                let player_sprite_sheet = browser::fetch_json_as::<models::Sheet>(
                    "/assets/sprite_sheets/rhb_trimmed.json",
                )
                .await?;

                let player_sheet_image =
                    engine::do_load_image("/assets/sprite_sheets/rhb_trimmed.png").await?;

                let game_background =
                    engine::do_load_image("assets/original/freetileset/png/BG/BG.png").await?;

                let game_stone =
                    engine::do_load_image("assets/original/freetileset/png/Object/Stone.png")
                        .await?;

                let rhb = avatar::RedHatBoy::new(player_sprite_sheet, player_sheet_image);

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy: rhb,
                    stone: engine::Image::new(game_stone, models::Point::new(150, 546)),
                    background: engine::Image::new(game_background, models::Point::new(0, 0)),
                })))
            }
            WalkTheDog::Loaded(_) => Err(anyhow!("Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        if let WalkTheDog::Loaded(walk) = self {
            if keystate.is_pressed("Space") {
                walk.boy.jump();
            }
            if keystate.is_pressed("ArrowDown") {
                walk.boy.slide();
            }
            if keystate.is_pressed("ArrowUp") {
                walk.boy.run_up();
            }
            if keystate.is_pressed("ArrowRight") {
                walk.boy.run_right();
            }
            if keystate.is_pressed("ArrowLeft") {
                walk.boy.run_left();
            }

            walk.boy.update();
            if walk
                .boy
                .bounding_box()
                .intersects(walk.stone.bounding_box())
            {
                log!("knockout occured!");
                walk.boy.knock_out();
            }
        }
    }

    fn draw(&self, renderer: &engine::Renderer) {
        renderer.clear(&models::Rect {
            x: 0.0,
            y: 0.0,
            width: 1200.0,
            height: 1200.0,
        });

        if let WalkTheDog::Loaded(walk) = self {
            walk.background.draw(renderer);
            walk.stone.draw(renderer);
            walk.boy.draw(renderer);
        }
    }
}
