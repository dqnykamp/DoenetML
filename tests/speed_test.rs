// // #![cfg(target_arch = "wasm32")]

// #[macro_use]
// mod common_node;
// use serde_json;
// use std::panic::set_hook;
// use std::{collections::HashMap, thread};
// use std::f64::NAN;

// use common_node::*;
// use doenet_core::{
//     parse_json::{DoenetMLWarning, DoenetMLError, RangeInDoenetML, SelfCloseRange, OpenCloseRange},
//     state_variables::StateVarValue,
// };
// use wasm_bindgen_test::{console_log, wasm_bindgen_test};


// #[wasm_bindgen_test]
// fn one_thousand_copies() {
//     static DATA: &str = r#"

//     <number name="n">1</number>

//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
    
//     <p>
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n 
//     $n $n $n $n $n $n $n $n $n $n
//     </p>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);
//     assert_sv_is_number(&dc, "n", "value", 1.0);


// }
