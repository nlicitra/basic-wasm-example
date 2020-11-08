mod utils;

use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn log_two(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("Howdy");
    log_u32(42);
    log_two("Hello", "world!");
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    RED,
    ORANGE,
    YELLOW,
    GREEN,
    BLUE,
    INDIGO,
    VIOLET,
}
// #[wasm_bindgen]
// impl Color {
//     #[wasm_bindgen(js_name = toString)]
//     pub fn to_string(&self) -> String {
//         let hex = match *self {
//             Color::RED => "#ef5350",
//             Color::ORANGE => "#ffb74d",
//             Color::YELLOW => "#fff176",
//             Color::GREEN => "#81c784",
//             Color::BLUE => "#64b5f6",
//             Color::INDIGO => "#7986cb",
//             Color::VIOLET => "#9575cd",
//         };
//         String::from(hex)
//     }
// }

#[wasm_bindgen]
pub struct Card {
    pub color: Color,
    pub number: u8,
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(number: u8, color: Color) -> Self {
        Self { number, color }
    }

    // #[wasm_bindgen(getter)]
    // pub fn number(&self) -> u8 {
    //     self.number
    // }
    // #[wasm_bindgen(getter)]
    // pub fn color(&self) -> Color {
    //     self.color
    // }
}
