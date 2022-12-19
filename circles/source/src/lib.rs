mod utils;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, firefly!");
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    // let body= document.body().unwrap();
    // body.set_inner_html("<canvas id='canvas' height='1200' width='2000'></canvas>");
    // let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

        context.begin_path();
        context.move_to(252.0, 150.0);
        context.arc(252.0, 150.0, 102.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#909090".into());
        context.fill();
        context.begin_path();
        context.move_to(558.0, 450.0);
        context.arc(558.0, 150.0, 108.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#aaaaaa".into());
        context.fill();
        context.begin_path();
        context.move_to(847.0, 750.0);
        context.arc(847.0, 150.0, 97.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#eeeeee".into());
        context.fill();
        context.begin_path();
        context.move_to(1153.0, 1050.0);
        context.arc(1153.0, 150.0, 103.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#909090".into());
        context.fill();
        context.begin_path();
        context.move_to(1473.0, 1350.0);
        context.arc(1473.0, 150.0, 123.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#f1f1f1".into());
        context.fill();
        context.begin_path();
        context.move_to(1766.0, 1650.0);
        context.arc(1766.0, 150.0, 116.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#E0e0e0".into());
        context.fill();
        context.begin_path();
        context.move_to(254.0, 1950.0);
        context.arc(254.0, 450.0, 104.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#eeeeee".into());
        context.fill();
        context.begin_path();
        context.move_to(564.0, 2250.0);
        context.arc(564.0, 450.0, 114.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#aaaaaa".into());
        context.fill();
        context.begin_path();
        context.move_to(801.0, 2550.0);
        context.arc(801.0, 450.0, 51.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#f1f1f1".into());
        context.fill();
        context.begin_path();
        context.move_to(1101.0, 2850.0);
        context.arc(1101.0, 450.0, 51.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#aaaaaa".into());
        context.fill();
        context.begin_path();
        context.move_to(1445.0, 3150.0);
        context.arc(1445.0, 450.0, 95.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#909090".into());
        context.fill();
        context.begin_path();
        context.move_to(1762.0, 3450.0);
        context.arc(1762.0, 450.0, 112.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#E0e0e0".into());
        context.fill();
        context.begin_path();
        context.move_to(198.0, 3750.0);
        context.arc(198.0, 750.0, 48.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#cccccc".into());
        context.fill();
        context.begin_path();
        context.move_to(555.0, 4050.0);
        context.arc(555.0, 750.0, 105.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#E0e0e0".into());
        context.fill();
        context.begin_path();
        context.move_to(860.0, 4350.0);
        context.arc(860.0, 750.0, 110.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#eeeeee".into());
        context.fill();
        context.begin_path();
        context.move_to(1166.0, 4650.0);
        context.arc(1166.0, 750.0, 116.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#909090".into());
        context.fill();
        context.begin_path();
        context.move_to(1445.0, 4950.0);
        context.arc(1445.0, 750.0, 95.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#cccccc".into());
        context.fill();
        context.begin_path();
        context.move_to(1699.0, 5250.0);
        context.arc(1699.0, 750.0, 49.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#E0e0e0".into());
        context.fill();
        context.begin_path();
        context.move_to(245.0, 5550.0);
        context.arc(245.0, 1050.0, 95.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#eeeeee".into());
        context.fill();
        context.begin_path();
        context.move_to(552.0, 5850.0);
        context.arc(552.0, 1050.0, 102.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#cccccc".into());
        context.fill();
        context.begin_path();
        context.move_to(798.0, 6150.0);
        context.arc(798.0, 1050.0, 48.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#aaaaaa".into());
        context.fill();
        context.begin_path();
        context.move_to(1167.0, 6450.0);
        context.arc(1167.0, 1050.0, 117.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#eeeeee".into());
        context.fill();
        context.begin_path();
        context.move_to(1464.0, 6750.0);
        context.arc(1464.0, 1050.0, 114.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#eeeeee".into());
        context.fill();
        context.begin_path();
        context.move_to(1775.0, 7050.0);
        context.arc(1775.0, 1050.0, 125.0, 0.0, f64::consts::PI * 2.0).unwrap();
        context.set_fill_style(&"#cccccc".into());
        context.fill();
        
}
