use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc;

use futures::channel::oneshot;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, Element, Event, HtmlElement, HtmlImageElement, HtmlInputElement, MouseEvent,
};

use crate::board::{convert_to_index, Board};
use crate::color::*;
use crate::moves::{get_next_move, GameResult, Move};
use crate::piece::Piece;
use crate::position::Position;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

thread_local! {
    // ONly allow select one square at the time
    static IS_SELECTING: Cell<bool> = Cell::new(false);
}

const ROW: usize = 5;
const COL: usize = 5;
const LVL: usize = 5;
const SQSIZE: usize = 6; //Square Size (vh)
const PLAYERCOLOR: Color = Color::White;

// Define a wrapper struct that holds the result
pub struct MenuInteractionResult {
    pub difficulty_value: i32,
}

impl MenuInteractionResult {
    fn new(difficulty_value: i32) -> Self {
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

fn get_selected_difficulty_value() -> i32 {
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

pub fn create_piece_image(id: &str) -> HtmlImageElement {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let img = document
        .create_element("img")
        .expect("failed to create element")
        .dyn_into::<HtmlImageElement>()
        .expect("failed to cast element");
    img.set_src(&format!(
        "https://raw.githubusercontent.com/WR104/chess-3D/main/www/img/{}.svg",
        id
    ));
    img
}

pub fn update_board(board: &Board, initial: bool) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.get_elements_by_class_name("square");
    let mut delay: f64 = 1.33; // delay of antimation fadeIn for initial update

    for (i, square) in board.squares().iter().enumerate() {
        let square_element = squares
            .item(i as u32)
            .expect("should have a square element")
            .dyn_into::<Element>()
            .expect("failed to cast element");

        while let Some(child) = square_element.first_child() {
            square_element
                .remove_child(&child)
                .expect("failed to remvoe child");
        }

        if let Some(piece) = square.get_piece() {
            let chess_color = match piece.get_color() {
                WHITE => "w",
                BLACK => "b",
            };

            let chess_type = piece.get_type();
            let chess_id = format!("{}{}", chess_color, chess_type);
            let img = create_piece_image(&chess_id);
            img.set_attribute("class", "piece")
                .expect("failed to set class attribute");

            // If this is the frist time, then apply fadeIn animation for each piece
            if initial {
                img.style()
                    .set_property("animation", &format!("fadeIn 1s ease {}s both", delay))
                    .expect("failed to set animation property");
                delay += 0.1;
            }
            square_element
                .append_child(&img)
                .expect("failed to append child");
        }
    }
}

pub async fn get_selected_square() -> Result<Position, &'static str> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    let sender = Rc::new(RefCell::new(Some(sender)));

    let closure = Closure::wrap(Box::new(move |event: Event| {
        IS_SELECTING.with(|is_selecting| {
            if !is_selecting.get() {
                is_selecting.set(true);

                let mouse_event = event.dyn_into::<MouseEvent>().unwrap();
                let target = mouse_event.target().unwrap();
                let square = if target.dyn_ref::<HtmlImageElement>().is_some() {
                    target
                        .dyn_into::<HtmlElement>()
                        .expect("Failed to cast target into an HtmlElement")
                        .parent_element()
                        .expect("Failed to get parent element")
                        .dyn_into::<HtmlElement>()
                        .expect("Failed to cast parent element into an Element")
                } else {
                    target
                        .dyn_into::<HtmlElement>()
                        .expect("Failed to cast target into an HtmlElement")
                };

                let i = square
                    .get_attribute("data-i")
                    .and_then(|i| i.parse::<i32>().ok());
                let j = square
                    .get_attribute("data-j")
                    .and_then(|j| j.parse::<i32>().ok());
                let k = square
                    .get_attribute("data-k")
                    .and_then(|k| k.parse::<i32>().ok());

                match (i, j, k) {
                    (Some(i), Some(j), Some(k)) => {
                        let position = Position::new(i, j, k);
                        if let Some(sender) = sender.borrow_mut().take() {
                            sender.send(Ok(position)).unwrap();
                        }
                    }
                    _ => {
                        if let Some(sender) = sender.borrow_mut().take() {
                            sender.send(Err("Invalid square")).unwrap();
                        }
                    }
                }

                is_selecting.set(false);
            }
        });
    }) as Box<dyn FnMut(_)>);

    let window = web_sys::window().expect("no global `window” exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.query_selector_all(".square").unwrap();

    for i in 0..squares.length() {
        if let Some(square) = squares.item(i) {
            square
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
        }
    }

    let position = receiver.await.unwrap();

    for i in 0..squares.length() {
        if let Some(square) = squares.item(i) {
            square
                .remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
        }
    }

    closure.forget();

    position
}

pub fn get_hint_pos(board: &Board, pos: Position) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();
    if let Some(piece) = board.get_piece(pos) {
        let moves = piece.get_legal_moves(board);
        for m in moves {
            match m {
                Move::Piece(_, to) | Move::Promotion(_, to, _) => {
                    result.push(to);
                }
                _ => {}
            }
        }
    }

    result
}

pub fn update_hint_squares(hint_pos: Vec<Position>) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let squares = document.get_elements_by_class_name("square");

    for pos in hint_pos {
        let index = convert_to_index(pos);

        let square_element = squares
            .item(index as u32)
            .expect("should have a square element")
            .dyn_into::<Element>()
            .expect("failed to cast element");

        let hint_box = document
            .create_element("div")
            .expect("failed to create element")
            .dyn_into::<Element>()
            .expect("failed to cast element");

        hint_box.set_class_name("hint");

        // Check if an <img> child is present
        if let Some(image_element) = square_element.query_selector("img").unwrap() {
            // Append the hint_box as the second child
            square_element
                .insert_before(&hint_box, Some(&image_element))
                .expect("failed to insert hint_box before img_element");
        } else {
            // Append the hint_box as the last child
            square_element
                .append_child(&hint_box)
                .expect("failed to append child");
        }
    }
}

pub fn render_loop(board: Rc<RefCell<Board>>, difficulty: i32) {
    let mut board_clone = Rc::clone(&board);

    if board.borrow().get_turn_color() == PLAYERCOLOR {
        // Selecting the piece the user wants to move
        let first_selected_square_future = get_selected_square();

        wasm_bindgen_futures::spawn_local(async move {
            let first_selected_square = first_selected_square_future.await;

            match first_selected_square {
                Ok(first_square) => {
                    let from = first_square;
                    // Get the hint squares
                    let hint_position = get_hint_pos(&board.borrow(), from);
                    if !hint_position.is_empty() {
                        update_hint_squares(hint_position);
                    }

                    // Wait for the user to select the second square
                    let second_selected_square = get_selected_square().await;

                    match second_selected_square {
                        Ok(second_square) => {
                            update_board(&board_clone.borrow(), false);

                            let to = second_square;

                            // Need to update the promotion feature
                            let m = match board.borrow().get_piece(from) {
                                Some(Piece::Pawn(_, _)) => Move::Piece(from, to),
                                _ => Move::Piece(from, to),
                            };

                            match board.borrow_mut().play_move(m) {
                                GameResult::Continuing(next_board) => {
                                    log!("Continuing");
                                    board_clone = Rc::new(RefCell::new(next_board));
                                }

                                GameResult::Victory(next_board, _) => {
                                    log!("You won the game!");
                                    board_clone = Rc::new(RefCell::new(next_board));
                                    update_board(&board_clone.borrow(), false);
                                    return;
                                }

                                GameResult::Stalemate(next_board) => {
                                    log!("Drawn Game");
                                    board_clone = Rc::new(RefCell::new(next_board));
                                    update_board(&board_clone.borrow(), false);
                                    return;
                                }

                                GameResult::IllegalMove(_) => {
                                    log!("IllegalMove");
                                }
                            }
                        }

                        Err(err) => {
                            log!("Error selecting second square: {}", err);
                        }
                    }
                }

                Err(err) => {
                    log!("Error selecting first square: {}", err);
                }
            }

            update_board(&board_clone.borrow(), false);
            render_loop(Rc::clone(&board_clone), difficulty)
        });
    } else {
        // Computer makes decisions
        let m = get_next_move(&board.borrow(), difficulty);

        match board.borrow_mut().play_move(m) {
            GameResult::Continuing(next_board) => {
                log!("Continuing");
                board_clone = Rc::new(RefCell::new(next_board));
            }

            GameResult::Victory(next_board, _) => {
                log!("You lost the game!");
                board_clone = Rc::new(RefCell::new(next_board));
                update_board(&board_clone.borrow(), false);
                return;
            }

            GameResult::Stalemate(next_board) => {
                log!("Drawn Game");
                board_clone = Rc::new(RefCell::new(next_board));
                update_board(&board_clone.borrow(), false);
                return;
            }

            GameResult::IllegalMove(_) => {
                log!("IllegalMove");
            }
        }
        update_board(&board_clone.borrow(), false);
        render_loop(Rc::clone(&board_clone), difficulty);
    }
}
