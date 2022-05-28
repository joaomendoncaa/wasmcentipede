mod centipede;
mod random;

use std::{cell::RefCell, rc::Rc};

use centipede::CentipedeGame;
use js_sys::Function;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, UnwrapThrowExt,
};
use web_sys::{window, HtmlDivElement, HtmlElement, KeyboardEvent};

use crate::centipede::Direction;

thread_local! {
    static GAME: Rc<RefCell<CentipedeGame>> = Rc::new(RefCell::new(CentipedeGame::new(20, 20)));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        || {
            GAME.with(|game| game.borrow_mut().tick());
            render();
        }
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new({
        |event: KeyboardEvent| {
            GAME.with(|game| {
                let new_direction = match &event.key()[..] {
                    "ArrowUp" => Some(Direction::Up),
                    "ArrowRight" => Some(Direction::Right),
                    "ArrowDown" => Some(Direction::Down),
                    "ArrowLeft" => Some(Direction::Left),
                    _ => None,
                  };

                if let Some(new_direction) = new_direction {
                    game.borrow_mut().update_direction(new_direction);
                };
            });
        }
    }) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn main() {
    let interval_ms: i32 = 225;

    TICK_CLOSURE.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                interval_ms,
            )
            .unwrap_throw()
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    let document = window().unwrap_throw().document().unwrap_throw();

    let root_container = document
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();

    root_container.set_inner_html("");

    let grid_width = GAME.with(|game| game.borrow().width);
    let grid_height = GAME.with(|game| game.borrow().height);

    root_container
        .style()
        .set_property("display", "inline-grid")
        .unwrap_throw();

    root_container
        .style()
        .set_property(
            "grid-template",
            &format!(
                "repeat({}, auto) / repeat({}, auto)",
                grid_width, grid_height
            ),
        )
        .unwrap_throw();

    for y in 0..grid_height {
        for x in 0..grid_width {
            let position = (x, y);

            let field_element = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();

            field_element.set_inner_text({
                if position == GAME.with(|game| game.borrow().insect_position) {
                    "🦋"
                } else if GAME.with(|game| game.borrow().centipede[0] == position) {
                    "👾"
                } else if GAME.with(|game| game.borrow().centipede.contains(&position)) {
                    "🟪"
                } else {
                    "⬜"
                }
            });

            root_container.append_child(&field_element).unwrap_throw();
        }
    }
}
