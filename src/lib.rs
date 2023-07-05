mod util;

use fractal_rs::{Config, Mandelbrot};
use num::Complex;
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, ImageData};

const FRACTAL_CANVAS: &'static str = "fractal_canvas";

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//#[wasm_bindgen(start)]
pub fn init() {
    util::set_panic_hook();
}

fn get_context() -> CanvasRenderingContext2d {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id(FRACTAL_CANVAS)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

#[wasm_bindgen]
pub fn render(
    width: u32,
    height: u32,
    cx: f64,
    cy: f64,
    zoom: f64,
    iter: u16,
) -> Result<(), JsValue> {
    let context = get_context();
    let c = Config {
        width,
        height,
        center: Complex::<f64>::new(cx, cy),
        zoom,
        iter,
        pix: fractal_rs::PixelFormat::RGBA8,
    };
    let mut m = Mandelbrot::new(c);
    let mut data = m.render();
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    context.put_image_data(&data, 0.0, 0.0)
}