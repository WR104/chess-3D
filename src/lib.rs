extern crate js_sys;
extern crate web_sys;

mod utils;
mod interface;

use wasm_bindgen::prelude::*;
use interface::*;
use wasm_bindgen_futures::JsFuture;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let menu_interaction_result = setup_menu_interaction().await;
    let difficulty = menu_interaction_result.difficulty_value;
    log!("d: {}" ,difficulty);
    create_boards();
    Ok(())
}



