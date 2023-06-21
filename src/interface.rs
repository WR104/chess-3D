use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    window, Element, Event, HtmlElement, HtmlImageElement, HtmlInputElement,
    HtmlTableElement, HtmlTableRowElement, HtmlTableSectionElement, MouseEvent, Screen,
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
const MINSCREENWIDTH: i32 = 600; // Minimal screen width

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
    let screen = window.screen().unwrap().dyn_into::<Screen>().unwrap();
    let screen_width = screen.width().unwrap();

    // The screen is too small to display long contents
    if screen_width <= MINSCREENWIDTH {
        // Change the mode content
        let mode_container = document
            .get_elements_by_class_name("modeContainer")
            .item(0)
            .expect("failed to get modeContainer");

        let h4_elements = mode_container
            .query_selector_all("h4")
            .expect("failed to execute querySelectorAll");

        h4_elements
            .item(0)
            .expect("no item 0")
            .dyn_into::<HtmlElement>()
            .expect("failed to cast to HTmlElement")
            .set_text_content(Some("Easy"));

        h4_elements
            .item(1)
            .expect("no item 1")
            .dyn_into::<HtmlElement>()
            .expect("failed to cast to HTmlElement")
            .set_text_content(Some("Normal"));

        h4_elements
            .item(2)
            .expect("no item 2")
            .dyn_into::<HtmlElement>()
            .expect("failed to cast to HTmlElement")
            .set_text_content(Some("Hard"));

        // Change the button content
        let button_element = document
            .get_element_by_id("submitButton")
            .expect("failed to get submitButton")
            .dyn_into::<HtmlElement>()
            .expect("failed to cast to HtmlElement");
        button_element
            .set_text_content(Some(&format!("Play\u{00A0}\u{25B6}")));
    }

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

            if chess_id == "wK" && PLAYERCOLOR == WHITE || chess_id == "bK" && PLAYERCOLOR == BLACK
            {
                if board.is_in_check(PLAYERCOLOR) {
                    img.set_attribute("id", "checked")
                        .expect("failed to set id attribute");
                }
            }

            // Turn on every module if the screen size is allowed
            if initial {
                let screen = window.screen().unwrap().dyn_into::<Screen>().unwrap();
                let screen_width = screen.width().unwrap();

                // Animation for every piece
                img.style()
                    .set_property("animation", &format!("fadeIn 1s ease {}s both", delay))
                    .expect("failed to set animation property");
                // Delay in animation showing piece
                delay += 0.1;

                // Display the play again button
                let button = document
                    .get_element_by_id("playAgain")
                    .expect("failed to get playAgain")
                    .dyn_into::<HtmlElement>()
                    .expect("failed to cast to HtmlElement");
                button
                    .style()
                    .set_property("display", "block")
                    .expect("failed to set property display");

                // minimum screen size
                if screen_width >= MINSCREENWIDTH {
                    // Display the moves history panel
                    let moves_panel = document
                        .get_element_by_id("movesPanel")
                        .expect("failed to get movesPanel")
                        .dyn_into::<HtmlElement>()
                        .expect("failed to cast to HtmlElement");
                    moves_panel
                        .style()
                        .set_property("display", "block")
                        .expect("failed to set property display");

                    // Diplay the catch Panel
                    let catch_panel = document
                        .get_element_by_id("catchPanel")
                        .expect("failed to get catchPanel")
                        .dyn_into::<HtmlElement>()
                        .expect("failed to cast to HtmlElement");
                    catch_panel
                        .style()
                        .set_property("display", "block")
                        .expect("failed to set property display");
                } else {
                    // Turn of the status
                    let status = document
                        .get_element_by_id("statusContainer")
                        .expect("should have status in document")
                        .dyn_into::<HtmlElement>()
                        .expect("failed to cast to HtmlElemet");

                    status
                        .style()
                        .set_property("display", "none")
                        .expect("failed to set display property");
                }
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

pub fn update_status(str1: &str, str2: &str) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let status = document
        .get_element_by_id("status")
        .expect("should have status in document");

    let new_status = format!("{} <br> {}", str1, str2);
    status.set_inner_html(&new_status);
}

pub fn get_chess_id(piece: Piece) -> String {
    let chess_color = match piece.get_color() {
        WHITE => "w",
        BLACK => "b",
    };
    let chess_type = piece.get_type();
    let chess_id = format!("{}{}", chess_color, chess_type);
    chess_id
}

pub fn update_moves_history(piece: Option<Piece>, m: &str) {
    if let Some(piece) = piece {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let table = document
            .get_element_by_id("movesBox")
            .expect("shuold have movesBox on document")
            .dyn_into::<HtmlTableElement>()
            .expect("failed to cast Element to HtmlTableElement");
        let tbody = table
            .t_bodies()
            .item(0)
            .unwrap()
            .dyn_into::<HtmlTableSectionElement>()
            .expect("failed to cast HtmlTableElement to HtmlTableSectionElement");

        let mut row_to_update = None;
        for i in 0..tbody.rows().length() {
            let row = tbody
                .rows()
                .item(i)
                .unwrap()
                .dyn_into::<HtmlTableRowElement>()
                .expect("failed to cast Element to HtmlTableRowElement");
            if row.cells().length() == 1 {
                row_to_update = Some(row);
                break;
            }
        }

        // Moves history displayed by pairs
        let row_to_update = match row_to_update {
            Some(row) => row,
            None => {
                let new_row = document
                    .create_element("tr")
                    .expect("failed to create tr")
                    .dyn_into::<HtmlTableRowElement>()
                    .expect("failed to cast Element to HtmlTableRowElement");
                tbody.append_child(&new_row).expect("failed to append tr");
                new_row
            }
        };

        let new_cell = document.create_element("td").expect("failed to create td");

        let chess_id = get_chess_id(piece);
        let img = create_piece_image(&chess_id);

        new_cell.append_child(&img).expect("failed to append child");
        new_cell.set_inner_html(&format!("{} {}", new_cell.inner_html(), m));

        row_to_update
            .append_child(&new_cell)
            .expect("failed to append child");
    }
}

pub fn update_catch_piece(piece: Option<Piece>, player: bool) {
    if let Some(piece) = piece {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let catch_id = if player {
            "playerCatch"
        } else {
            "computerCatch"
        };
        let catch_target = document
            .get_element_by_id(catch_id)
            .expect("failed to get Element");
        let catch_box = catch_target
            .get_elements_by_class_name("catchBox")
            .item(0)
            .expect("failed to get catchBox");

        let chess_type = piece.get_type();
        let chess_id = get_chess_id(piece);
        let img = create_piece_image(&chess_id);
        img.set_attribute("chessType", &chess_type)
            .expect("failed to set attribute chessType");

        let existing_img = catch_box
            .query_selector_all(&format!("[chessType='{}']", chess_type))
            .expect("failed to excute querSelector");
        let last_existing_img = existing_img.item(existing_img.length() - 1);

        // Make the pieces with same types overlapping
        if let Some(last_existing_img) = last_existing_img {
            img.style()
                .set_property("margin", "0px 0px 0px -25px")
                .expect("failed to set property margin");

            last_existing_img
                .dyn_into::<Element>()
                .expect("failed to cast to Element")
                .insert_adjacent_element("afterend", &img)
                .expect("failed to insert adjacent element");
        } else {
            catch_box
                .append_child(&img)
                .expect("failed to append child");
        }
    }
}

pub fn update_eval(rate: f64) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let rate_bar = document
        .get_element_by_id("rate")
        .expect("failed to get element rate")
        .dyn_into::<HtmlElement>()
        .expect("failed to cast to HtmlElement");

    rate_bar
        .style()
        .set_property("width", &format!("{}%", &rate.to_string()))
        .expect("failed to set property width");

    // update the score
    let score_element = document
        .get_element_by_id("score")
        .expect("failed to get score");
    let score = (rate - 50.0) / 5.0;
    let score = (score * 10.0).round().mul_add(0.1, 0.0);
    score_element.set_inner_html(&format!("<b>EVAL</b> {:.2}", score));
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
                    // Convert the player turn to str, then update it
                    let color_str_ref: &str = &PLAYERCOLOR.to_string();
                    update_status(color_str_ref, "TO MOVE");

                    let from = first_square;
                    let selected_piece = board.borrow().get_piece(from);

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
                            // Might catch a piece here
                            let catched_piece = board.borrow().get_piece(to);

                            // Need to update the promotion feature
                            let m = match board.borrow().get_piece(from) {
                                Some(Piece::Pawn(_, _)) => Move::Piece(from, to),
                                _ => Move::Piece(from, to),
                            };

                            match board.borrow_mut().play_move(m) {
                                GameResult::Continuing(next_board) => {
                                    board_clone = Rc::new(RefCell::new(next_board));
                                    let rate = next_board.eval_value(PLAYERCOLOR);
                                    update_eval(rate);
                                    update_moves_history(selected_piece, &m.to_string());
                                    update_catch_piece(catched_piece, true);
                                }

                                GameResult::Victory(next_board, _) => {
                                    update_status("YOU", "WON!");
                                    board_clone = Rc::new(RefCell::new(next_board));
                                    update_board(&board_clone.borrow(), false);
                                    return;
                                }

                                GameResult::Stalemate(next_board) => {
                                    update_status("DRAWN", "GAME");
                                    board_clone = Rc::new(RefCell::new(next_board));
                                    update_board(&board_clone.borrow(), false);
                                    return;
                                }

                                GameResult::IllegalMove(_) => {
                                    update_status("ILLEGAL", "MOVE");
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
        // Convert the computer's turn to str, then update it
        let color_str_ref: &str = &PLAYERCOLOR.to_string();
        update_status(color_str_ref, "TO MOVE");

        // Computer makes decisions
        let m = get_next_move(&board.borrow(), difficulty);
        let (from, to) = match m {
            Move::Piece(from, to) => (from, to),
            Move::Promotion(from, to, _) => (from, to),
            Move::Resign => (Position::new(6, 6, 6), Position::new(6, 6, 6)),
        };
        let moved_piece = board.borrow().get_piece(from);
        // Might catch a piece here
        let catched_piece = board.borrow().get_piece(to);

        match board.borrow_mut().play_move(m) {
            GameResult::Continuing(next_board) => {
                board_clone = Rc::new(RefCell::new(next_board));
                let rate = next_board.eval_value(PLAYERCOLOR);
                update_eval(rate);
                update_moves_history(moved_piece, &m.to_string());
                update_catch_piece(catched_piece, false);
            }

            GameResult::Victory(next_board, _) => {
                update_status("YOU", "LOST!");
                board_clone = Rc::new(RefCell::new(next_board));
                update_board(&board_clone.borrow(), false);
                return;
            }

            GameResult::Stalemate(next_board) => {
                update_status("DRAWN", "GAME");
                board_clone = Rc::new(RefCell::new(next_board));
                update_board(&board_clone.borrow(), false);
                return;
            }

            GameResult::IllegalMove(_) => {}
        }
        update_board(&board_clone.borrow(), false);
        render_loop(Rc::clone(&board_clone), difficulty);
    }
}
