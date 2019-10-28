#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
fn display(text: &str) {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p").expect("failed to create a <p>");
    val.set_inner_html(text);

    body.append_child(&val).expect("failed to append child to body");
}

#[cfg(not(target_arch = "wasm32"))]
fn display(text: &str) {
    println!("{}", text);
}

fn main() {
    display("Hello from Rust!");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    main();
}
