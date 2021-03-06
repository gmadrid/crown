#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;

macro_rules! app_runner {
    ($func:ident, $app_class:path) => {
        #[wasm_bindgen]
        pub fn $func() -> Result<(), JsValue> {
            yew::start_app::<$app_class>();
            Ok(())
        }
    };
}

mod arc;
mod crownapp;
mod point;

pub use crate::crownapp::CrownApp;

app_runner!(run_crown_app, CrownApp);
