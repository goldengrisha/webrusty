use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebSocket;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn ws_ping(endpoint: String, message: String) -> Result<(), JsValue> {
    // Create the WebSocket connection
    let web_socket = WebSocket::new(endpoint.as_str())?;
    log(&format!("WebSocket created: {}", web_socket.url()));

    let cloned_ws = web_socket.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");

        match cloned_ws.send_with_str(message.as_str()) {
            Ok(_) => log("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    });
    web_socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}
