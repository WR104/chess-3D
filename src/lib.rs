extern crate js_sys;
extern crate web_sys;

mod utils;

use web_sys::{window, Element, HtmlElement };
use wasm_bindgen::prelude::*;

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

const ROW: usize = 5;
const COL: usize = 5;
const LVL: usize = 5;
const SQSIZE: usize = 6;    //Square Size (vh)

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    log!("start");

    create_boards();
    Ok(())
}

fn create_boards() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let board_container = document 
        .get_elements_by_class_name("board-container")
        .item(0)
        .expect("should have a chessboard element");

    for k in (0..LVL).rev() {
        let board = document.create_element("div").expect("failed to create board");
        board.set_class_name("board");

        for i in (0..ROW).rev() {
            for j in 0..COL {
                let square = document
                    .create_element("div")
                    .expect("failed to create element")
                    .dyn_into::<Element>()
                    .expect("failed to cast element");
                
                square.set_class_name("square");
                square
                    .class_list()
                    .add_1(if (i + j + k) % 2 != 0{
                        "lightSq"
                    } else {
                        "darkSq"
                    })
                    .unwrap();

                //convert the i & j to row and col of the chess board
                let row = i;
                let col = j;
                let lvl = k;
                square
                    .set_attribute("data-i", &row.to_string())
                    .expect("failed to set data-i attribute");
                square
                    .set_attribute("data-j", &col.to_string())
                    .expect("failed to set data-j attribute");
                square
                    .set_attribute("data-k", &lvl.to_string())
                    .expect("failed to set data-k attribute");

                board.append_child(&square).unwrap();
            }
        }

        board_container.append_child(&board).unwrap();
    }

    let all_boards = document.get_elements_by_class_name("board");
    for i in 0..all_boards.length() {
        let lvl = all_boards
            .item(i)
            .expect("should have a board element")
            .dyn_into::<HtmlElement>()
            .expect("failed to cast element");

        let height = SQSIZE * LVL;
        let missing = height as f64 * (1.0 - (63.0 * std::f64::consts::PI / 180.0).cos()) * 0.95;

        lvl.style()
            .set_property("height", &format!("{}vh", height))
            .unwrap();
        lvl.style()
            .set_property("width", &format!("{}vh", height))
            .unwrap();

        lvl.style()
            .set_property(
                "transform",
                &format!(
                    "translateY({}vh) rotateX(63deg) skew(336deg)",
                    -(i as f64) * missing
                ),
            )
            .unwrap();

    }
}

