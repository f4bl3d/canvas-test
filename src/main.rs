#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
mod canvas_state;

use std::cell::RefCell;
use std::cmp::{max, min};
use std::ops::Div;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlCanvasElement, HtmlDivElement, HtmlElement};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> dioxus::core::Element {
    let window = window().expect("couldnt find html window");
    let document = window.document().expect("document not found");
    let canvas = document.get_element_by_id("mycanvas").unwrap();

    cx.render(rsx!(rsx! {
    div {id: "canvasdiv",
                   canvas {
                id: "mycanvas",
           width: "500",
           height: "500",
           // other attributes and styles
       }}
    }))
}
