mod utils;

extern crate web_sys;

use wasm_bindgen::prelude::*;

use doenet_core::{DoenetCore};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide println! style syntax for console.log logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// Raw module means that this relative path is based on the wasm file's location
// -#[wasm_bindgen(raw_module = "/src/Core/CoreWorker.js")]
// -extern "C" {
// -    fn logJson(label: &str, json_obj: String);
// -}

// // Raw module means that this relative path is based on the wasm file's location
// #[wasm_bindgen(module = "compiled_parser/parser")]
// extern "C" {
//     pub fn parseAndCompile(in_text: String) -> JsValue;
// }



#[wasm_bindgen(getter_with_clone)]
#[derive(Debug)]
pub struct PublicDoenetCore(DoenetCore, pub js_sys::Array, pub js_sys::Array);



#[wasm_bindgen]
impl PublicDoenetCore {
    /// Create components from JSON tree and create all dependencies.
    pub fn new(program: &str) -> Result<PublicDoenetCore, String> {

        utils::set_panic_hook();

        web_sys::console::time_with_label("DoenetCore creation");
                
        let core_or_error = doenet_core::create_doenet_core(program, None);

        web_sys::console::time_end_with_label("DoenetCore creation");

        match core_or_error {
            Err(doenet_ml_error) => Err(doenet_ml_error.to_string()),
            Ok((core, ml_warnings, ml_errors)) => {
                let warnings_array = js_sys::Array::new();
                for (i, warning) in ml_warnings.iter().enumerate() {
                    warnings_array.set(i as u32, JsValue::from(warning.to_string()));
                }
                let errors_array = js_sys::Array::new();
                for (i, error) in ml_errors.iter().enumerate() {
                    errors_array.set(i as u32, JsValue::from(error.to_string()));
                }
                Ok(PublicDoenetCore(core, warnings_array, errors_array))
            }
        }
    }   


    // pub fn display_all_state(&self) -> String {
    //     serde_json::to_string(&doenet_core::utils::json_components(
    //         &self.0.component_nodes,
    //         &self.0.component_states
    //     )).unwrap_or_default()
    // }


    pub fn update_renderers(&self) -> String {
        web_sys::console::time_with_label("Update renderers");

        let result = doenet_core::update_renderers(&self.0);
        web_sys::console::time_end_with_label("Update renderers");

        result
    }



    pub fn handle_action(&mut self, action: &str) -> String {

        let completed_action_id = doenet_core::handle_action_from_json(&mut self.0, action);
        completed_action_id
    }

}
