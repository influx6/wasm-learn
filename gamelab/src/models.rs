use anyhow::Result;
use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::HtmlImageElement;

use crate::browser;
use crate::engine;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SheetRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub frame: SheetRect,
    pub sprite_source_size: SheetRect,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sheet {
    pub frames: HashMap<String, Cell>,
}

pub enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
