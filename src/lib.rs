use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn tada() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document
        .get_element_by_id("tada")
        .expect("document should have a body");

    // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");
    let text_node = document.create_text_node("Hello From Rust!!!");

    body.append_child(&text_node)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
