extern crate js_sys;
extern crate web_sys;

mod utils;
mod interface;
mod piece;
mod position;
mod color;
mod square;
mod board;
mod moves;
mod weights;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use interface::*;
use crate::board::Board;

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
    let board = Rc::new(RefCell::new(Board::new()));
    let menu_interaction_result = setup_menu_interaction().await;
    let difficulty = menu_interaction_result.difficulty_value;
    log!("d: {}" ,difficulty);
    create_boards();

    update_board(&board.borrow(), false);
    
    render_loop(Rc::clone(&board));
    Ok(())
}



