mod centipede;
mod random;

use std::{cell::RefCell, rc::Rc};

use centipede::CentipedeGame;
use js_sys::Function;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, UnwrapThrowExt,
};
use web_sys::window;

thread_local! {
    static GAME: Rc<RefCell<CentipedeGame>> = Rc::new(RefCell::new(CentipedeGame::new(20, 20)));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || game.borrow_mut().tick()
    }) as Box<dyn FnMut()>);
}

#[wasm_bindgen(start)]
pub fn main() {
    TICK_CLOSURE.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                500,
            )
            .unwrap_throw()
    });
}

// TODO: render game on screen with web::sys
pub fn render() {}
