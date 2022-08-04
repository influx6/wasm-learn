#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s : &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s : &str);

    #[wasm_bindgen(module = "./index")]
    fn stats_updated(stats : JsValue);

    pub type Display;

    #[wasm_bindgen(method, structural, js_namespace = ROT)]
    fn draw(this: &Display, x: i32, y: i32, ch: &str);

    #[wasm_bindgen(method, structural, js_namespace = draw, js_namespace = ROT)]
    fn draw_color(this: &Display, x: i32, y: i32, ch: &str, color: &str);
}

#[wasm_bindgen]
pub struct GridPoint {
    pub x: i32
    pub y: i32
}

#[wasm_bindgen]
pub struct Engine {
    display: Display,
    points: HashMap<GridPoint, String>,
    prize_location: Option<GridPoint>
}

#[derive(Serialize)]
pub struct Stats {
    pub hits: i32
}

#[wasm_bindgen]
pub struct PlayerCore {
    display: Display,
}

impl PlayerCore {
    #[wasm_bindgen(constructor)]
    pub fn new(display: Display) -> Self {
        PlayerCore {
            display,
        }
    }

    pub fn emit_stat(&self) {
        let stat = Stats {
            hits: 20,
        };
        stats_updated(JsValue::from_serde(&stat).unwrap());
    }
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(display: Display) -> Self {
        Engine {
            display,
            points: HashMap::new(),
            prize_location: None,
        }
    }

    pub fn on_dig(&mut self, x: i32, y: i32, val: i32) {
        if val == 0 {
            let pt = GridPoint{x, y};
            self.points.insert(pt, ".".to_owned());
        }
    }

    pub fn draw_map(&self) {
        for (k, v) in &self.points {
            self.display.draw(k.x, k.y, &v);
        }
    }
}