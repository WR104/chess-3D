use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use futures::channel::oneshot;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{window, Element, Event, HtmlElement, HtmlInputElement};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const ROW: usize = 5;
const COL: usize = 5;
const LVL: usize = 5;
const SQSIZE: usize = 6; //Square Size (vh)

// Define a wrapper struct that holds the result
pub struct MenuInteractionResult {
    pub difficulty_value: u32,
}

impl MenuInteractionResult {
    fn new(difficulty_value: u32) -> Self {
        Self { difficulty_value }
    }
}

// Show the menu
// Disappear when the user click the paly button
// Return the difficulty level the user chose
pub async fn setup_menu_interaction() -> MenuInteractionResult {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let submit_button = document
        .get_element_by_id("submitButton")
        .expect("Submit button not found")
        .dyn_into::<HtmlElement>()
        .expect("Failed to convert to HtmlElement");

    let (sender, receiver) = futures::channel::oneshot::channel();

    let closure = Closure::once(move |_event: Event| {
        hide_menu();
        let difficulty_value = get_selected_difficulty_value();
        let _ = sender.send(MenuInteractionResult::new(difficulty_value));
    });

    submit_button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Failed to add event listener");

    closure.forget();

    // Await the result from the channel and return it
    receiver.await.expect("Failed to receive result")
}

fn hide_menu() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let menu = document
        .get_element_by_id("menu")
        .expect("Menu element not found")
        .dyn_into::<HtmlElement>()
        .expect("failed to cast element");
    menu.style()
        .set_property("display", "none")
        .expect("Failed to hide menu");
}

fn get_selected_difficulty_value() -> u32 {
    let difficulty_inputs = vec!["easy", "normal", "hard"];
    let document = web_sys::window()
        .expect("Window not found")
        .document()
        .expect("Document not found");

    for input_id in difficulty_inputs {
        let input = document
            .get_element_by_id(input_id)
            .expect(&format!("Input element with id '{}' not found", input_id))
            .dyn_into::<HtmlInputElement>()
            .expect("Failed to cast element to HtmlInputElement");
        if input.checked() {
            return input.value().parse().unwrap_or(0);
        }
    }

    0 // Default difficulty value
}

pub fn create_boards() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let board_container = document
        .get_elements_by_class_name("board-container")
        .item(0)
        .expect("should have a chessboard element");

    for k in (0..LVL).rev() {
        let board = document
            .create_element("div")
            .expect("failed to create board");
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
                    .add_1(if (i + j + k) % 2 != 0 {
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
