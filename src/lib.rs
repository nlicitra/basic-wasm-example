mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
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

    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    SPADE,
    CLUB,
    HEART,
    DIAMOND,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u8, suit: Suit) -> Self {
        Self { value, suit }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

#[wasm_bindgen]
impl Deck {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let cards = vec![];
        let mut deck = Self { cards };
        deck.init();
        deck
    }

    pub fn init(&mut self) {
        self.cards.clear();
        for suit in &[Suit::CLUB, Suit::DIAMOND, Suit::HEART, Suit::SPADE] {
            for number in 1..14 {
                self.cards.push(Card::new(number, *suit))
            }
        }
        self.shuffle();
    }

    pub fn get_cards(&self) -> JsValue {
        JsValue::from_serde(&self.cards).unwrap()
    }

    pub fn deal(&mut self, num_of_cards: usize) -> JsValue {
        if self.cards.is_empty() {
            alert("Re-shuffling the deck ♻");
            self.init();
        }
        let split_point = match self.cards.len() < num_of_cards {
            true => 0,
            false => self.cards.len() - num_of_cards,
        };
        JsValue::from_serde(&self.cards.split_off(split_point)).unwrap()
    }

    pub fn shuffle(&mut self) {
        log("Shuffling the deck...");
        self.cards.shuffle(&mut thread_rng());
    }
}
