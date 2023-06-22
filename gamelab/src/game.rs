use anyhow::Result;
use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::HtmlImageElement;

use crate::browser;
use crate::engine;

#[derive(Deserialize, Serialize, Debug)]
struct SheetRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize, Serialize, Debug)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    position: engine::Point,
    sheet: Option<Sheet>,
    frame: u8,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog {
            image: None,
            sheet: None,
            frame: 0,
            position: engine::Point { x: 0, y: 0 },
        }
    }
}

#[async_trait(?Send)]
impl engine::Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        let player_sprite_sheet =
            browser::fetch_json_as::<Sheet>("/assets/sprite_sheets/rhb.json").await?;

        let player_sheet_image = engine::do_load_image("/assets/sprite_sheets/rhb.png").await?;

        Ok(Box::new(WalkTheDog {
            sheet: Some(player_sprite_sheet),
            image: Some(player_sheet_image),
            position: self.position,
            frame: self.frame,
        }))
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        let mut velocity = engine::Point { x: 0, y: 0 };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3;
        }
        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3;
        }
        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3;
        }
        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3;
        }

        self.position.x += velocity.x;
        self.position.y += velocity.y;

        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }

    fn draw(&self, renderer: &engine::Renderer) {
        // self.frame = (self.frame + 1) % 8;
        let current_sprite = (self.frame / 3) + 1;
        let frame_name = format!("Run ({}).png", current_sprite);

        let sprite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");

        renderer.clear(&engine::Rect {
            x: 0.0,
            y: 0.0,
            width: 1200.0,
            height: 1200.0,
        });

        let size = engine::Rect {
            x: sprite.frame.x.into(),
            y: sprite.frame.y.into(),
            width: sprite.frame.w.into(),
            height: sprite.frame.h.into(),
        };

        let location = engine::Rect {
            x: self.position.x.into(),
            y: self.position.y.into(),
            width: sprite.frame.w.into(),
            height: sprite.frame.h.into(),
        };

        self.image.as_ref().map(|image| {
            renderer.draw_image(&image, &size, &location);
        });
    }
}
