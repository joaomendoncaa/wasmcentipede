#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(target_family = "wasm"))]
pub fn random_range(min: usize, max: usize, inclusive: bool) -> usize {
    let mut rng = thread_rng();

    if inclusive {
        return rng.gen_range(min..=max);
    }

    return rng.gen_range(min..max);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[cfg(target_family = "wasm")]
pub fn random_range(min: usize, max: usize, inclusive: bool) -> usize {
    return (random() * (max - min) as f64).floor() as usize + min;
}
