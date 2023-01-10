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

// // ========= DoenetML errrors ============


// #[wasm_bindgen_test]
// fn doenet_ml_error_invalid_tag() {
//     static DATA: &str = r#"
//     <p>Invalid tag in p: <invalidTag1 /></p>
//     Invalid tag outside p <invalidTag2 />
//     <text>Invalid tag inside text: <invalidTag3 /></text>

//     <text name="t">This text works</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(warnings, vec![]);

//     assert_eq!(
//         errors,
//         vec![
//             DoenetMLError::InvalidComponentType {
//                 comp_type: "invalidTag1".to_string(),
//                 doenetml_range:  RangeInDoenetML::SelfClose(SelfCloseRange { self_close_begin:27, self_close_end: 38})
//             },
//             DoenetMLError::InvalidComponentType {
//                 comp_type: "invalidTag2".to_string(),
//                 doenetml_range:  RangeInDoenetML::SelfClose(SelfCloseRange { self_close_begin:73, self_close_end: 84})
//             },
//             DoenetMLError::InvalidComponentType {
//                 comp_type: "invalidTag3".to_string(),
//                 doenetml_range:  RangeInDoenetML::SelfClose(SelfCloseRange { self_close_begin:124, self_close_end: 135})
//             },
//         ]
//     );

//     assert_sv_is_string(&dc, "t", "value", "This text works");

//     assert_sv_is_string(&dc, "/__error1", "message", "Component type invalidTag1 does not exist. Found at indices 27-38.");
//     assert_sv_is_string(&dc, "/__error2", "message", "Component type invalidTag2 does not exist. Found at indices 73-84.");
//     assert_sv_is_string(&dc, "/__error3", "message", "Component type invalidTag3 does not exist. Found at indices 124-135.");

//     // text component is not created as it cannot display errors
//     assert!(dc.component_states.get("/_text1").is_none());

// }

// #[wasm_bindgen_test]
// fn doenet_ml_error_invalid_attribute() {
//     static DATA: &str = r#"<text invalidAttr="hmm" />
//     <text name="t">This text works</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(warnings, vec![]);


//     // TODO: self_close_begin is wrong as parser doesn't calculate that correctly!!!
//     // Have wrong value for now so that test passes
//     assert_eq!(
//         errors,
//         vec![
//             DoenetMLError::AttributeDoesNotExist {
//                 comp_name: "/_text1".to_string(),
//                 attr_name: "invalidattr".to_string(),
//                 doenetml_range:  RangeInDoenetML::SelfClose(SelfCloseRange { self_close_begin:6, self_close_end: 24})
//             },
//         ]
//     );

//     assert_sv_is_string(&dc, "t", "value", "This text works");

//     assert_sv_is_string(&dc, "/__error1", "message", "Attribute 'invalidattr' does not exist on /_text1. Found at indices 6-24.");

//     // text component is not created
//     assert!(dc.component_states.get("/_text1").is_none());

// }

// #[wasm_bindgen_test]
// fn doenet_ml_error_invalid_component_name() {
//     static DATA: &str = r#"<text name="_text1" /><text name="text:2" />
    
//     <text name="t">This text works</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(warnings, vec![]);


//     // TODO: self_close_begin is wrong as parser doesn't calculate that correctly!!!
//     // Have wrong value for now so that test passes
//     assert_eq!(
//         errors,
//         vec![
//             DoenetMLError::InvalidComponentName {
//                 name: "_text1".to_string(),
//                 doenetml_range:  RangeInDoenetML::SelfClose(SelfCloseRange { self_close_begin:6, self_close_end: 20})
//             },
//             DoenetMLError::InvalidComponentName {
//                 name: "text:2".to_string(),
//                 doenetml_range:  RangeInDoenetML::SelfClose(SelfCloseRange { self_close_begin:28, self_close_end: 42})
//             },
//         ]
//     );

//     assert_sv_is_string(&dc, "t", "value", "This text works");

//     assert_sv_is_string(&dc, "/__error1", "message", "The component name _text1 is invalid.  It must begin with a letter and can contain only letters, numbers, hyphens, and underscores. Found at indices 6-20.");
//     assert_sv_is_string(&dc, "/__error2", "message", "The component name text:2 is invalid.  It must begin with a letter and can contain only letters, numbers, hyphens, and underscores. Found at indices 28-42.");

//     // text component is not created
//     assert!(dc.component_states.get("_text1").is_none());
//     assert!(dc.component_states.get("text:2").is_none());

// }

// #[wasm_bindgen_test]
// fn doenet_ml_error_duplicate_component_name() {
//     static DATA: &str = r#"<text name="t1">Original</text><text name="t1" />
    
//     <text name="t2">This text works</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(warnings, vec![]);


//     // TODO: self_close_begin is wrong as parser doesn't calculate that correctly!!!
//     // Have wrong value for now so that test passes
//     assert_eq!(
//         errors,
//         vec![
//             DoenetMLError::DuplicateName {
//                 name: "t1".to_string(),
//                 doenetml_range:  RangeInDoenetML::SelfClose(SelfCloseRange { self_close_begin:37, self_close_end: 47})
//             },
//         ]
//     );

//     assert_sv_is_string(&dc, "t1", "value", "Original");
//     assert_sv_is_string(&dc, "t2", "value", "This text works");

//     assert_sv_is_string(&dc, "/__error1", "message", "The component name t1 is used multiple times. Found at indices 37-47.");

//     // text component is not created
//     assert!(dc.component_states.get("/_text1").is_none());

// }

// #[wasm_bindgen_test]
// fn doenet_ml_error_cannot_add_error_component() {
//     static DATA: &str = r#"<_error>Cannot add directly</_error>
    
//     <text name="t2">This text works</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(warnings, vec![]);


//     // TODO: self_close_begin is wrong as parser doesn't calculate that correctly!!!
//     // Have wrong value for now so that test passes
//     assert_eq!(
//         errors,
//         vec![
//             DoenetMLError::InvalidComponentType {
//                 comp_type: "_error".to_string(),
//                 doenetml_range:  RangeInDoenetML::OpenClose(OpenCloseRange {open_begin: 1, open_end: 7, close_begin: 27, close_end: 36 })
//             },
//         ]
//     );

//     assert_sv_is_string(&dc, "t2", "value", "This text works");

//     assert_sv_is_string(&dc, "/__error1", "message", "Component type _error does not exist. Found at indices 1-36.");

//     // text component is not created
//     assert!(dc.component_states.get("/_text1").is_none());

// }

// #[wasm_bindgen_test]
// fn doenet_ml_error_cyclic_dependency_through_children_indirectly() {
//     static DATA: &str = r#"
//         <text name='a_parent'><text name='a' copySource='b'/></text>
//         <text name='b'><text name='b_child' copySource='a_parent'/></text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let error = doenet_core_from(DATA).unwrap_err();
//     assert!(matches!(
//         error,
//         DoenetMLError::CyclicalDependency {
//             component_chain: _,
//             doenetml_range: _
//         }
//     ));
// }



// // =========== DoenetML warnings ===========


// #[wasm_bindgen_test]
// fn doenet_ml_warning_copy_nonexistent_component_gives_warning() {
//     static DATA: &str = r#"
//         <text copySource='qwerty' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(
//         warnings,
//         vec![DoenetMLWarning::ComponentDoesNotExist {
//             comp_name: "qwerty".to_string(),
//             doenetml_range:  RangeInDoenetML::None
//         }]
//     );

//     assert_eq!(errors, vec![]);

//     assert_sv_is_string(&dc, "/_text1", "value", "");

// }

// #[wasm_bindgen_test]
// fn doenet_ml_warning_copy_nonexistent_state_var_gives_warning() {
//     static DATA: &str = r#"
//         <text name='a'>hi</text>
//         <text copySource='a' copyProp='qwertyqwerty' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(
//         warnings,
//         vec![DoenetMLWarning::StateVarDoesNotExist {
//             comp_name: "a".into(),
//             sv_name: "qwertyqwerty".into(),
//             doenetml_range: RangeInDoenetML::None
//         }]
//     );

//     assert_eq!(errors, vec![]);

//     assert_sv_is_string(&dc, "a", "value", "hi");
//     assert_sv_is_string(&dc, "/_text2", "value", "");

// }

// #[wasm_bindgen_test]
// fn doenet_ml_error_cannot_use_copy_info_as_prop() {
//     static DATA: &str = r#"
//         <sequence name='s' from='0' to='2' />
//         <number name='n' copySource='s' copyProp='value' propIndex='2'/>
//         <number copySource='n' copyProp='propIndex' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(
//         warnings,
//         vec![DoenetMLWarning::StateVarDoesNotExist {
//             comp_name: "n".into(),
//             sv_name: "propIndex".into(),
//             doenetml_range: RangeInDoenetML::None
//         }]
//     );

//     assert_eq!(errors, vec![]);

//     assert_sv_is_number(&dc, "n", "value", 1.0);
//     assert_sv_is_number(&dc, "/_number2", "value", 0.0);


// }

// #[wasm_bindgen_test]
// fn doenet_ml_warning_prop_index_not_positive_integer() {
//     static DATA: &str = r#"
//     <sequence name='s' from='1' to='5' />
//     <number copySource='s' copyProp='value' propIndex='1.5' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(
//         warnings,
//         vec![DoenetMLWarning::PropIndexIsNotPositiveInteger {
//             comp_name: "/_number1".to_string(),
//             invalid_index: "1.5".to_string(),
//             doenetml_range: RangeInDoenetML::None,
//         }]
//     );

//     assert_eq!(errors, vec![]);

//     assert_sv_is_number(&dc, "/_number1", "value", NAN);

// }

// // ========= <text> ==============

// #[wasm_bindgen_test]
// fn text_preserves_spaces_between_text_tags() {
//     static DATA: &str = r#"
//         <text name='a'><text>Hello</text> <text>there</text>!</text>
//         <text name='b'><text>We <text>could</text> be <text copySource="/_text3" />.</text></text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "a", "value", "Hello there!");
//     assert_sv_is_string(&dc, "b", "value", "We could be there.");
// }

// #[wasm_bindgen_test]
// fn text_inside_text() {
//     static DATA: &str = r#"
//         <text>one<text> two <text name='t2' copySource='t' /> <text name='t'>three</text> again </text><text copySource="t2"/> once more</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(
//         &dc,
//         "/_text1",
//         "value",
//         "one two three three again three once more",
//     );
// }

// #[wasm_bindgen_test]
// fn text_copy_component_of_copy_component() {
//     static DATA: &str = r#"
//         <text name='a'><text name='one'>one</text></text>
//         <text name='b' copySource='a'><text name='two'>two</text></text>
//         <text name='c' copySource='b'><text name='three'>three</text></text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "a", "text", "one");
//     assert_sv_is_string(&dc, "b", "text", "onetwo");
//     assert_sv_is_string(&dc, "c", "text", "onetwothree");
// }

// #[wasm_bindgen_test]
// fn text_copy_component_cyclical_gives_error() {
//     static DATA: &str = r#"
//         <text name='irrelevant' copySource='a' />
//         <text name='a' copySource='b' />
//         <text name='b' copySource='a' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let error = doenet_core_from(DATA).unwrap_err();
//     assert!(matches!(
//         error,
//         DoenetMLError::CyclicalDependency {
//             component_chain: _,
//             doenetml_range: _
//         }
//     ));
// }

// #[wasm_bindgen_test]
// fn text_copy_itself_as_child_gives_error() {
//     static DATA: &str = r#"
//         <text name='t'> $t</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let error = doenet_core_from(DATA).unwrap_err();
//     assert!(matches!(
//         error,
//         DoenetMLError::CyclicalDependency {
//             component_chain: _,
//             doenetml_range: _
//         }
//     ));
// }

// #[wasm_bindgen_test]
// fn text_copy_itself_as_grandchild_gives_error() {
//     static DATA: &str = r#"
//         <text name='t'><text>$t</text></text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let error = doenet_core_from(DATA).unwrap_err();
//     match error {
//         DoenetMLError::CyclicalDependency {
//             component_chain,
//             doenetml_range: _,
//         } => assert_eq!(component_chain.len(), 3),
//         _ => panic!("Wrong error type"),
//     };
// }

// #[wasm_bindgen_test]
// fn text_copy_prop_and_copy_source_combinations_with_additional_children() {
//     static DATA: &str = r#"
//     <text name='ti'>hi</text>

//     <p>a: <text name='a' copySource='ti' copyProp='value' /></p>
//     <p>b: <text name='b' copySource='a' /></p>
//     <p>c: <text name='c' copySource='a'> more text</text></p>
//     <p>d: <text name='d'>$a more text</text></p>
//     <p>e: <text name='e'>$a.value more text</text></p>
//     <p>f: <text name='f' copySource='b' /></p>
//     <p>g: <text name='g' copySource='ti' copyProp='value'> more text</text></p>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "a", "value", "hi");
//     assert_sv_is_string(&dc, "b", "value", "hi");
//     assert_sv_is_string(&dc, "c", "value", "hi more text");
//     assert_sv_is_string(&dc, "d", "value", "hi more text");
//     assert_sv_is_string(&dc, "e", "value", "hi more text");
//     assert_sv_is_string(&dc, "f", "value", "hi");
//     assert_sv_is_string(&dc, "g", "value", "hi more text");
// }

// // ========= <textInput> ==============

// #[wasm_bindgen_test]
// fn text_input_update_immediate_value_and_update_value() {
//     static DATA: &str = r#"
//         <textInput />

//         <!-- Make sure this change also affects copies -->

//         <textInput copySource='/_textInput1' />
//         <textInput copySource='/_textInput2' />

//         <text copySource='/_textInput3' copyProp='immediateValue' />
//         <text copySource='/_textInput3' copyProp='value' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     update_immediate_value_for_text(&dc, "/_textInput1", "this is the new immediate value");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(
//         &dc,
//         "/_textInput1",
//         "immediateValue",
//         "this is the new immediate value",
//     );
//     assert_sv_is_string(
//         &dc,
//         "/_textInput2",
//         "immediateValue",
//         "this is the new immediate value",
//     );
//     assert_sv_is_string(
//         &dc,
//         "/_textInput3",
//         "immediateValue",
//         "this is the new immediate value",
//     );
//     assert_sv_is_string(&dc, "/_text1", "value", "this is the new immediate value");

//     // Now updateValue
//     doenet_core::handle_action(
//         &mut dc,
//         doenet_core::Action {
//             component_name: String::from("/_textInput1"),
//             action_name: String::from("updateValue"),
//             args: HashMap::new(),
//         },
//     );
//     doenet_core::update_renderers(&dc);

//     // Note that the other textinput's value sv's are still stale because only the shared essential
//     // data has changed
//     assert_sv_is_string(
//         &dc,
//         "/_textInput3",
//         "value",
//         "this is the new immediate value",
//     );
//     assert_sv_is_string(&dc, "/_text2", "value", "this is the new immediate value");

//     // Make sure that if we change the other textInputs, the essential data will still change
//     update_immediate_value_for_text(
//         &dc,
//         "/_textInput1",
//         "the second text input changed this value",
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(
//         &dc,
//         "/_textInput1",
//         "immediateValue",
//         "the second text input changed this value",
//     );
//     assert_sv_is_string(
//         &dc,
//         "/_textInput2",
//         "immediateValue",
//         "the second text input changed this value",
//     );
//     assert_sv_is_string(
//         &dc,
//         "/_textInput3",
//         "immediateValue",
//         "the second text input changed this value",
//     );
//     assert_sv_is_string(
//         &dc,
//         "/_text1",
//         "value",
//         "the second text input changed this value",
//     );
// }

// #[wasm_bindgen_test]
// fn text_input_macro() {
//     static DATA: &str = r#"
//         <textInput name="t" prefill="Cake"/>
//         <text>$t.value is good.</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "t", "value", "Cake");
//     assert_sv_is_string(&dc, "t", "immediateValue", "Cake");
//     assert_sv_is_string(&dc, "/_text1", "value", "Cake is good.");
// }

// // ========= <numberInput> ==============

// #[wasm_bindgen_test]
// fn number_input_immediate_value_syncs_with_value_on_update_request() {
//     static DATA: &str = r#"
//     <numberinput name='the_number_input'/>
//     <graph name="g">
//         <point name='myPoint' xs='$the_number_input.value 3-2' />
//     </graph>
//     <number name='myNum' copySource='the_number_input' copyProp='immediateValue' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     // assert_sv_array_is_number_list(&dc, "myPoint", "xs", vec![0.0, 1.0]);

//     move_point_2d(
//         &dc,
//         "myPoint",
//         StateVarValue::Number(-5.11),
//         StateVarValue::Number(27.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "myPoint", "xs", vec![-5.11, 27.0]);
//     assert_sv_is_number(&dc, "the_number_input", "value", -5.11);
//     assert_sv_is_number(&dc, "the_number_input", "immediateValue", -5.11);
//     assert_sv_is_number(&dc, "myNum", "value", -5.11);
// }

// #[wasm_bindgen_test]
// fn number_input_value_remains_nan_until_update_value() {
//     static DATA: &str = r#"
//     <numberInput name='n'/>
//     $n.value
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "n", "rawRendererValue", "");
//     assert_sv_is_number(&dc, "n", "immediateValue", f64::NAN);
//     assert_sv_is_number(&dc, "n", "value", f64::NAN);

//     update_immediate_value_for_number(&dc, "n", "13.0");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "n", "rawRendererValue", "13.0");
//     assert_sv_is_number(&dc, "n", "immediateValue", 13.0);
//     assert_sv_is_number(&dc, "n", "value", f64::NAN);

//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "n", "rawRendererValue", "13.0");
//     assert_sv_is_number(&dc, "n", "immediateValue", 13.0);
//     assert_sv_is_number(&dc, "n", "value", 13.0);
// }

// #[wasm_bindgen_test]
// fn number_input_raw_renderer_value_not_overriden_on_update_value_action() {
//     static DATA: &str = r#"
//     <numberInput name='n'/>
//     $n.immediateValue $n.value
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);

//     update_immediate_value_for_number(&dc, "n", "non numerical value");
//     doenet_core::update_renderers(&dc);
//     assert_sv_is_string(&dc, "n", "rawRendererValue", "non numerical value");
//     assert_sv_is_number(&dc, "n", "immediateValue", f64::NAN);
//     assert_sv_is_number(&dc, "n", "value", f64::NAN);

//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);
//     assert_sv_is_string(&dc, "n", "rawRendererValue", "non numerical value");
//     assert_sv_is_number(&dc, "n", "immediateValue", f64::NAN);
//     assert_sv_is_number(&dc, "n", "value", f64::NAN);
// }

// #[wasm_bindgen_test]
// fn number_input_raw_renderer_value_updates_with_bind() {
//     static DATA: &str = r#"
//     <numberInput name='n'/>
//     <graph>
//         <point name='immediatePoint' xs='1 $n.immediateValue' />
//         <point name='valuePoint' xs='2 $n.value' />
//     </graph>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "n", "rawRendererValue", "");
//     assert_sv_is_number(&dc, "n", "immediateValue", f64::NAN);
//     assert_sv_is_number(&dc, "n", "value", f64::NAN);
//     assert_sv_array_is_number_list(&dc, "immediatePoint", "xs", vec![1.0, f64::NAN]);
//     assert_sv_array_is_number_list(&dc, "valuePoint", "xs", vec![2.0, f64::NAN]);

//     move_point_2d(
//         &dc,
//         "immediatePoint",
//         StateVarValue::Number(1.0),
//         StateVarValue::Number(4.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "n", "rawRendererValue", "4");
//     assert_sv_is_number(&dc, "n", "immediateValue", 4.0);
//     assert_sv_array_is_number_list(&dc, "immediatePoint", "xs", vec![1.0, 4.0]);
//     // assert_sv_is_number(&dc, "n", "value", f64::NAN);
//     // assert_sv_array_is_number_list(&dc, "valuePoint", "xs", vec![2.0, f64::NAN]);

//     move_point_2d(
//         &dc,
//         "valuePoint",
//         StateVarValue::Number(2.0),
//         StateVarValue::Number(-7.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "n", "rawRendererValue", "-7");
//     assert_sv_is_number(&dc, "n", "immediateValue", -7.0);
//     assert_sv_is_number(&dc, "n", "value", -7.0);
//     assert_sv_array_is_number_list(&dc, "immediatePoint", "xs", vec![1.0, -7.0]);
//     assert_sv_array_is_number_list(&dc, "valuePoint", "xs", vec![2.0, -7.0]);
// }

// // ========= <collect> =============

// #[wasm_bindgen_test]
// fn collect_and_copy_number_input_changes_original() {
//     static DATA: &str = r#"
//         <section name="inputs">
//                 <textinput name="input1" prefill="yolo"/>
//                 <textinput name="input2" prefill="3"/>
//         </section>

//         <collect componentType="textinput" source="inputs"/>

//         <section copySource="inputs"/>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     let render_tree_string = doenet_core::update_renderers(&dc);
//     let render_tree = serde_json::from_str(&render_tree_string).unwrap();

//     let collect1 = child_instructions_for(
//         &render_tree,
//         "/_document1",
//         "__textInput_from_(/_collect1[1])",
//     )
//     .get("actions")
//     .unwrap()
//     .as_object()
//     .unwrap()
//     .get("updateValue")
//     .unwrap()
//     .as_object()
//     .unwrap()
//     .get("componentName")
//     .unwrap()
//     .as_str()
//     .unwrap();
//     let collect2 = child_instructions_for(
//         &render_tree,
//         "/_document1",
//         "__textInput_from_(/_collect1[2])",
//     )
//     .get("actions")
//     .unwrap()
//     .as_object()
//     .unwrap()
//     .get("updateImmediateValue")
//     .unwrap()
//     .as_object()
//     .unwrap()
//     .get("componentName")
//     .unwrap()
//     .as_str()
//     .unwrap();
//     let copy1 = child_instructions_for(&render_tree, "/_section2", "__cp:input1(/_section2)")
//         .get("actions")
//         .unwrap()
//         .as_object()
//         .unwrap()
//         .get("updateImmediateValue")
//         .unwrap()
//         .as_object()
//         .unwrap()
//         .get("componentName")
//         .unwrap()
//         .as_str()
//         .unwrap();
//     let copy2 = child_instructions_for(&render_tree, "/_section2", "__cp:input2(/_section2)")
//         .get("actions")
//         .unwrap()
//         .as_object()
//         .unwrap()
//         .get("updateValue")
//         .unwrap()
//         .as_object()
//         .unwrap()
//         .get("componentName")
//         .unwrap()
//         .as_str()
//         .unwrap();

//     assert_eq!(collect1, "input1");
//     assert_eq!(collect2, "input2");
//     assert_eq!(copy1, "input1");
//     assert_eq!(copy2, "input2");

//     assert_sv_is_string(&dc, "input1", "immediateValue", "yolo");
//     assert_sv_is_string(&dc, "input2", "immediateValue", "3");
// }

// #[wasm_bindgen_test]
// fn collect_point_into_text() {
//     static DATA: &str = r#"
//         <graph name="graph">
//                 <point name="p1" xs="2 3"/>
//                 <point name="p2" xs="$p1.y $p1.x"/>
//         </graph>
//         <text name="t"><collect source="graph" componentType="point"/></text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p1", "xs", vec![2.0, 3.0]);
//     assert_sv_array_is_number_list(&dc, "p2", "xs", vec![3.0, 2.0]);
//     assert_sv_is_string(&dc, "t", "value", "(2, 3)(3, 2)");
// }

// #[wasm_bindgen_test]
// fn collect_sequence_changing() {
//     static DATA: &str = r#"
//         <number name="n" copySource="/_numberInput1" copyProp="value"/>:

//         <p name="p1">
//         <sequence name="seq" from="$n" to="$n+5"/>
//         </p>

//         <collect name="c1" source="p1" componentType="number"/>.

//         $seq[3].value
//         $c1[3].value
//         <numberInput prefill="6"/>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "__mcr:c1:value(/_document1)_1", "value", 8.0);
//     assert_sv_is_number(&dc, "__mcr:seq:value(/_document1)_1", "value", 8.0);

//     update_immediate_value_for_number(&dc, "/_numberInput1", "30");
//     update_value_for_number(&dc, "/_numberInput1");

//     // console_log!("the update: {:?}", doenet_core::utils::json_components(&dc.component_nodes, &dc.component_states));
//     assert_state_var_stale(
//         &dc,
//         "seq",
//         &vec![],
//         &doenet_core::state_variables::StateRef::ArrayElement("value", 3),
//     );
//     assert_state_var_stale(
//         &dc,
//         "__mcr:seq:value(/_document1)_1",
//         &vec![],
//         &doenet_core::state_variables::StateRef::Basic("value"),
//     );

//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_numberInput1", "value", 30.0);
//     assert_sv_array_is_number_list(
//         &dc,
//         "seq",
//         "value",
//         vec![30.0, 31.0, 32.0, 33.0, 34.0, 35.0],
//     );
//     assert_sv_is_number(&dc, "__mcr:c1:value(/_document1)_1", "value", 32.0);
//     assert_sv_is_number(&dc, "__mcr:seq:value(/_document1)_1", "value", 32.0);
// }

// // ========= <sequence> ==============

// #[wasm_bindgen_test]
// fn sequence_copies_component() {
//     static DATA: &str = r#"
//         <number name='f'>5</number>
//         <number name='t'>11</number>

//         <sequence name='s' from="$f" to="$t" />

//         <sequence copySource='s' />
//         <sequence copySource='s' from='3' to='6' />
//         <sequence copySource='s' from='9' />
//         <sequence copySource='s' from='300' />
//         <sequence copySource='s' to='$f' />
//         <sequence copySource='s' from='21' to='22' />

//         <!-- This sequence should be empty -->
//         <sequence copySource='s' to='-10' />

//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(
//         &dc,
//         "/_sequence2",
//         "value",
//         vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
//     );
//     assert_sv_array_is_number_list(&dc, "/_sequence3", "value", vec![3.0, 4.0, 5.0, 6.0]);
//     assert_sv_array_is_number_list(&dc, "/_sequence4", "value", vec![9.0, 10.0, 11.0]);
//     assert_sv_array_is_number_list(&dc, "/_sequence5", "value", vec![]);
//     assert_sv_array_is_number_list(&dc, "/_sequence6", "value", vec![5.0]);
//     assert_sv_array_is_number_list(&dc, "/_sequence7", "value", vec![21.0, 22.0]);
//     assert_sv_array_is_number_list(&dc, "/_sequence8", "value", vec![]);
// }

// #[wasm_bindgen_test]
// fn sequence_can_grow_and_shrink() {
//     static DATA: &str = r#"
//     <numberInput name='ni' />
//     <sequence to='$ni.value' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     update_immediate_value_for_number(&dc, "ni", "8.0");
//     update_value_for_number(&dc, "ni");
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(
//         &dc,
//         "/_sequence1",
//         "value",
//         vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
//     );

//     update_immediate_value_for_number(&dc, "ni", "9");
//     update_value_for_number(&dc, "ni");
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(
//         &dc,
//         "/_sequence1",
//         "value",
//         vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
//     );

//     update_immediate_value_for_number(&dc, "ni", "2.0");
//     update_value_for_number(&dc, "ni");
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "/_sequence1", "value", vec![1.0, 2.0]);

//     update_immediate_value_for_number(&dc, "ni", "asdf");
//     update_value_for_number(&dc, "ni");
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "/_sequence1", "value", vec![]);

//     update_immediate_value_for_number(&dc, "ni", "-3");
//     update_value_for_number(&dc, "ni");
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "/_sequence1", "value", vec![]);
// }

// #[wasm_bindgen_test]
// fn sequence_from_and_to_can_be_copied_as_props() {
//     static DATA: &str = r#"
//         <number name='f'>-1000</number>
//         <number name='t'>-993</number>

//         <sequence name='s' from="$f" to="$t" />

//         <number copySource='s' copyProp='from' />
//         <number copySource='s' copyProp='to' />
//         <number>$s.from</number>
//         <number>$s.to</number>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number3", "value", -1000.0);
//     assert_sv_is_number(&dc, "/_number4", "value", -993.0);
//     assert_sv_is_number(&dc, "/_number5", "value", -1000.0);
//     assert_sv_is_number(&dc, "/_number6", "value", -993.0);
// }

// #[wasm_bindgen_test]
// fn sequence_index_copied_based_on_number_input() {
//     static DATA: &str = r#"
//     <sequence name='s' from='10' to='15' />
//     <p><numberInput name='n' /></p>
//     <p><number copySource='s' copyProp='value' propIndex='$n.value'/></p>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     update_immediate_value_for_number(&dc, "n", "5.0");
//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number1", "value", 14.0);

//     update_immediate_value_for_number(&dc, "n", "2");
//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number1", "value", 11.0);
// }

// #[wasm_bindgen_test]
// fn sequence_macro_component_index() {
//     static DATA: &str = r#"
//         <p>
//                 <numberInput name="n" prefill="1"/>
//                 <sequence name="seq" from="$n.value" to="20"/>.

//                 <text>Fifth:$seq[5].value.</text>
//                 <text>Fifth:$seq[  5 ].value.</text>
//                 <text>Fifth: $seq[ 5 ].value</text>
//                 <text>Fifth: $seq[5  ].value</text>
//         </p>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);
//     assert_sv_is_string(&dc, "/_text1", "value", "Fifth:5.");
//     assert_sv_is_string(&dc, "/_text2", "value", "Fifth:5.");
//     assert_sv_is_string(&dc, "/_text3", "value", "Fifth: 5");
//     assert_sv_is_string(&dc, "/_text4", "value", "Fifth: 5");

//     update_immediate_value_for_number(&dc, "n", "6");
//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "/_text1", "value", "Fifth:10.");
//     assert_sv_is_string(&dc, "/_text2", "value", "Fifth:10.");
//     assert_sv_is_string(&dc, "/_text3", "value", "Fifth: 10");
//     assert_sv_is_string(&dc, "/_text4", "value", "Fifth: 10");
// }

// #[wasm_bindgen_test]
// fn sequence_empty() {
//     static DATA: &str = r#"
//     <sequence />
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "/_sequence1", "value", vec![]);
// }

// #[wasm_bindgen_test]
// fn sequence_dynamic_length() {
//     static DATA: &str = r#"
//     <numberinput name="n" prefill="4"/>
//     <text name="t"><sequence from="1" to="$n.value"/></text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "n", "value", 4.0);
//     assert_sv_is_string(&dc, "t", "value", "1234");

//     update_immediate_value_for_number(&dc, "n", "10.0");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "n", "immediateValue", 10.0);
//     assert_sv_is_number(&dc, "n", "value", 4.0);
//     assert_sv_is_string(&dc, "t", "value", "1234");

//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "n", "immediateValue", 10.0);
//     assert_sv_is_number(&dc, "n", "value", 10.0);
//     assert_sv_is_string(&dc, "t", "value", "12345678910");

//     update_immediate_value_for_number(&dc, "n", "8.0");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "n", "immediateValue", 8.0);
//     assert_sv_is_number(&dc, "n", "value", 10.0);
//     assert_sv_is_string(&dc, "t", "value", "12345678910");

//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "n", "immediateValue", 8.0);
//     assert_sv_is_number(&dc, "n", "value", 8.0);
//     assert_sv_is_string(&dc, "t", "value", "12345678");
// }

// // ========= <point> ==============

// #[wasm_bindgen_test]
// fn point_moves_copy_number() {
//     static DATA: &str = r#"
//         <number name='num'>2</number>
//         <graph name='g'><point name='p' xs='3 $num'/></graph>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p", "xs", vec![3.0, 2.0]);

//     move_point_2d(
//         &dc,
//         "p",
//         StateVarValue::Integer(5),
//         StateVarValue::Number(1.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p", "xs", vec![5.0, 1.0]);
// }

// #[wasm_bindgen_test]
// fn point_copies_coords_of_another_point() {
//     static DATA: &str = r#"
//     <graph>
//         <point name='a' xs='1 2' />
//         <point name='b' xs='$a.xs[1]-4 $a.xs[2]+2' />
//     </graph>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "a", "xs", vec![1.0, 2.0]);
//     assert_sv_array_is_number_list(&dc, "b", "xs", vec![-3.0, 4.0]);

//     move_point_2d(
//         &dc,
//         "a",
//         StateVarValue::Number(-2.0),
//         StateVarValue::Number(-5.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "a", "xs", vec![-2.0, -5.0]);
//     assert_sv_array_is_number_list(&dc, "b", "xs", vec![-6.0, -3.0]);
// }

// #[wasm_bindgen_test]
// fn point_copies_another_point_component() {
//     static DATA: &str = r#"
//     <graph><point name='p1' xs='1 2' /></graph>
//     <graph><point name='p2' copySource='p1' /></graph>
//     <graph><point name='p3' copySource='p2' /></graph>
//     <graph><point name='p4' copySource='p3' /></graph>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p1", "xs", vec![1.0, 2.0]);
//     assert_sv_array_is_number_list(&dc, "p2", "xs", vec![1.0, 2.0]);
//     assert_sv_array_is_number_list(&dc, "p3", "xs", vec![1.0, 2.0]);
//     assert_sv_array_is_number_list(&dc, "p4", "xs", vec![1.0, 2.0]);

//     move_point_2d(
//         &dc,
//         "p2",
//         StateVarValue::Number(-3.2),
//         StateVarValue::Number(7.1),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p1", "xs", vec![-3.2, 7.1]);
//     assert_sv_array_is_number_list(&dc, "p2", "xs", vec![-3.2, 7.1]);
//     assert_sv_array_is_number_list(&dc, "p3", "xs", vec![-3.2, 7.1]);
//     assert_sv_array_is_number_list(&dc, "p4", "xs", vec![-3.2, 7.1]);
// }

// #[wasm_bindgen_test]
// fn point_used_with_prop_index() {
//     static DATA: &str = r#"
//     <sequence name='s' from='10' to='15' />
//     <number name='id'>2</number>
//     <number copySource='s' copyProp='value' propIndex='$id.value' />
    
//     <graph>
//     <point name='p' xs='1 $id.value' />
//     </graph>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number2", "value", 11.0);

//     move_point_2d(
//         &dc,
//         "p",
//         StateVarValue::Integer(5),
//         StateVarValue::Number(4.123123),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number2", "value", f64::NAN);

//     move_point_2d(
//         &dc,
//         "p",
//         StateVarValue::Integer(5),
//         StateVarValue::Integer(4),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number2", "value", 13.0);
// }

// // ========= Map ===========
// #[wasm_bindgen_test]
// fn map_complicated_sources() {
//     static DATA: &str = r#"
//         <number name="n1">4</number>
//         <number name="n2">3</number>
//         <map>
//             <sources componentType="number" alias="i">
//                 <number name="n3">3</number>
//                 <number copySource="n1"/>
//                 <sequence from="1" to="$n2"/>
//                 <number>$n3 + 5</number>
//                 <number copySource = "/_sequence1" componentIndex="2" copyProp="value"/>
//             </sources>
//             <template>
//                 <text name="t">$i squared is <number>$i^2</number></text>
//             </template>
//         </map>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string_with_map(&dc, "t", vec![1], "value", "3 squared is 9");
//     assert_sv_is_string_with_map(&dc, "t", vec![2], "value", "4 squared is 16");
//     assert_sv_is_string_with_map(&dc, "t", vec![3], "value", "1 squared is 1");
//     assert_sv_is_string_with_map(&dc, "t", vec![4], "value", "2 squared is 4");
//     assert_sv_is_string_with_map(&dc, "t", vec![5], "value", "3 squared is 9");
//     assert_sv_is_string_with_map(&dc, "t", vec![6], "value", "8 squared is 64");
//     assert_sv_is_string_with_map(&dc, "t", vec![7], "value", "2 squared is 4");
// }

// #[wasm_bindgen_test]
// fn maps_in_maps() {
//     static DATA: &str = r#"
//     <map>
//         <sources componentType="number" alias="x">
//             <sequence from="1" to ="3"/>
//         </sources>
//         <template>
//             <map>
//                 <sources componentType="number" alias="y">
//                     <sequence from="1" to="$x"/>
//                 </sources>
//                 <template>
//                     <map>
//                         <sources componentType="text" alias="z">
//                             <text>$x is x</text>
//                             <text>$y is y</text>
//                             <text>rose is red</text>
//                         </sources>
//                         <template>
//                                 <text name="t">$z using $x and $y</text>
//                         </template>
//                     </map>
//                 </template>
//             </map>
//         </template>
//     </map>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![1], "value", 1);
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 1, 1], "value", "1 is x using 1 and 1");
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 1, 2], "value", "1 is y using 1 and 1");

//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![2], "value", 2);
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 1, 1], "value", "2 is x using 2 and 1");
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 1, 2], "value", "1 is y using 2 and 1");
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 2, 1], "value", "2 is x using 2 and 2");
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 2, 2], "value", "2 is y using 2 and 2");

//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![3], "value", 3);
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 1, 1], "value", "3 is x using 3 and 1");
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 1, 2], "value", "1 is y using 3 and 1");
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 2, 1], "value", "3 is x using 3 and 2");
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 2, 2], "value", "2 is y using 3 and 2");
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 3, 1], "value", "3 is x using 3 and 3");
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 3, 2], "value", "3 is y using 3 and 3");
// }

// #[wasm_bindgen_test]
// fn map_dynamic_size() {
//     static DATA: &str = r#"
//     <numberinput name="i" prefill="1"/>
//     <map>
//         <sources componentType="number" alias="one">
//             <sequence from="1" to="$i.value"/>
//         </sources>
//         <template>
//             <numberinput name="j" prefill="2"/>
//             <map>
//                 <sources componentType="number" alias="two">
//                     <sequence from="1" to="$j.value"/>
//                 </sources>
//                 <template>
//                     <text name="t">($one, $two) with size ($i.value, $j.value)</text>
//                 </template>
//             </map>
//         </template>
//     </map>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_size_is_with_map(&dc, "/_sequence1", vec![], "value", 1);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![1], "value", 2);
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 1], "value", "(1, 1) with size (1, 2)");
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 2], "value", "(1, 2) with size (1, 2)");

//     update_immediate_value_for_number(&dc, "i", "3.0");
//     doenet_core::update_renderers(&dc);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence1", vec![], "value", 1);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![1], "value", 2);

//     update_value_for_number(&dc, "i");
//     doenet_core::update_renderers(&dc);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence1", vec![], "value", 3);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![1], "value", 2);
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 1], "value", "(1, 1) with size (3, 2)");
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 2], "value", "(1, 2) with size (3, 2)");
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![2], "value", 2);
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 1], "value", "(2, 1) with size (3, 2)");
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 2], "value", "(2, 2) with size (3, 2)");
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![3], "value", 2);
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 1], "value", "(3, 1) with size (3, 2)");
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 2], "value", "(3, 2) with size (3, 2)");

//     let action_name = r#"[2]j"#;
//     update_immediate_value_for_number(&dc, action_name, "4.0");
//     doenet_core::update_renderers(&dc);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![2], "value", 2);

//     update_value_for_number(&dc, action_name);
//     doenet_core::update_renderers(&dc);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence1", vec![], "value", 3);
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![1], "value", 2);
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 1], "value", "(1, 1) with size (3, 2)");
//     assert_sv_is_string_with_map(&dc, "t", vec![1, 2], "value", "(1, 2) with size (3, 2)");
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![2], "value", 4);
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 1], "value", "(2, 1) with size (3, 4)");
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 2], "value", "(2, 2) with size (3, 4)");
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 3], "value", "(2, 3) with size (3, 4)");
//     assert_sv_is_string_with_map(&dc, "t", vec![2, 4], "value", "(2, 4) with size (3, 4)");
//     assert_sv_array_size_is_with_map(&dc, "/_sequence2", vec![3], "value", 2);
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 1], "value", "(3, 1) with size (3, 2)");
//     assert_sv_is_string_with_map(&dc, "t", vec![3, 2], "value", "(3, 2) with size (3, 2)");
// }

// #[wasm_bindgen_test]
// fn map_move_points() {
//     static DATA: &str = r#"
//     <map>
//     <sources componentType="number" alias="x">
//         <number>5</number>
//         <number>3</number>
//     </sources>
//     <template>
//             <graph>
//                     <point name="p" xs="$x 2"/>
//             </graph>
//     </template>
//     </map>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list_with_map(&dc, "p", vec![1], "numericalXs", vec![5.0, 2.0]);
//     assert_sv_array_is_number_list_with_map(&dc, "p", vec![2], "numericalXs", vec![3.0, 2.0]);

//     let first_instance = r#"[1]p"#;
//     move_point_2d(
//         &dc,
//         first_instance,
//         StateVarValue::Integer(2),
//         StateVarValue::Integer(4),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list_with_map(&dc, "p", vec![1], "numericalXs", vec![5.0, 4.0]);
//     assert_sv_array_is_number_list_with_map(&dc, "p", vec![2], "numericalXs", vec![3.0, 2.0]);

//     let second_instance = r#"[2]p"#;
//     move_point_2d(
//         &dc,
//         second_instance,
//         StateVarValue::Integer(1),
//         StateVarValue::Integer(6),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list_with_map(&dc, "p", vec![1], "numericalXs", vec![5.0, 4.0]);
//     assert_sv_array_is_number_list_with_map(&dc, "p", vec![2], "numericalXs", vec![3.0, 6.0]);
// }

// #[wasm_bindgen_test]
// fn map_inside_text() {
//     static DATA: &str = r#"
//     <text><map><sources alias="t" componentType="text">
//             <text>cow</text>
//             <text>horse</text>
//     </sources><template>some $t but <map>
//     <sources alias="r" componentType="text">
//             <text>yes</text>
//             <text>no</text>
//     </sources><template>then it answers $r </template>
//     </map>and </template>
//     </map>they left</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "/_text1", "value", "some cow but then it answers yes then it answers no and some horse but then it answers yes then it answers no and they left");
// }

// // ========= <conditionalContent> ===========

// #[wasm_bindgen_test]
// fn conditional_content_updating() {
//     static DATA: &str = r#"
//     <numberinput name="n" prefill="2"/>
//     <text>Description: <conditionalContent>
//         <case condition="$n.value>=0.0">positive, </case>
//         <case condition="$n.value<0.0">negative, </case>
//         <case condition="$n.value>2.0">greater than 2, </case>
//         <case condition="$n.value>1.0">greater than 1, </case>
//         <case condition="$n.value<3.0">less than 3, </case>
//     </conditionalContent>ok.</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(
//         &dc,
//         "/_text1",
//         "value",
//         "Description: positive, greater than 1, less than 3, ok.",
//     );

//     update_immediate_value_for_number(&dc, "n", "10");
//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);
//     assert_sv_is_string(
//         &dc,
//         "/_text1",
//         "value",
//         "Description: positive, greater than 2, greater than 1, ok.",
//     );

//     update_immediate_value_for_number(&dc, "n", "1");
//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);
//     assert_sv_is_string(
//         &dc,
//         "/_text1",
//         "value",
//         "Description: positive, less than 3, ok.",
//     );

//     update_immediate_value_for_number(&dc, "n", "-1");
//     update_value_for_number(&dc, "n");
//     doenet_core::update_renderers(&dc);
//     assert_sv_is_string(
//         &dc,
//         "/_text1",
//         "value",
//         "Description: negative, less than 3, ok.",
//     );
// }

// // =========== <boolean> ===========

// #[wasm_bindgen_test]
// fn boolean_operations() {
//     static DATA: &str = r#"
//         <booleanInput name="bool2"/>
//         <text hide="$bool2.value">Yin</text>
//         <text hide="!$bool2.value">Yang</text>

//         <number name="num">3</number>
//         <boolean>$num == 3.0</boolean>
//         <boolean>$num != 1.0</boolean>
//         <boolean>$num != 3.0</boolean>

//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_boolean(&dc, "/_text1", "hidden", false);
//     assert_sv_is_boolean(&dc, "/_text2", "hidden", true);

//     assert_sv_is_boolean(&dc, "/_boolean1", "value", true);
//     assert_sv_is_boolean(&dc, "/_boolean2", "value", true);
//     assert_sv_is_boolean(&dc, "/_boolean3", "value", false);
// }

// // =========== <line> ============

// #[wasm_bindgen_test]
// fn line_points_collection() {
//     static DATA: &str = r#"
//         <graph>
//                 <line p1="5 2" p2="3 4"/>
// 	        <point copySource="/_line1" copyCollection="points" componentIndex="1"/>
// 	        <point copySource="/_line1" copyCollection="points" componentIndex="2"/>
//         </graph>
//         <number copySource="/_line1" copyCollection="points" componentIndex="1" copyProp="xs" propIndex="2"/>
//         <number copySource="/_line1" copyCollection="points" componentIndex="2" copyProp="xs" propIndex="1"/>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number1", "value", 2.0);
//     assert_sv_is_number(&dc, "/_number2", "value", 3.0);
//     assert_sv_array_is_number_list(&dc, "/_point1", "numericalXs", vec![5.0, 2.0]);
//     assert_sv_array_is_number_list(&dc, "/_point2", "numericalXs", vec![3.0, 4.0]);

//     move_point_2d(
//         &dc,
//         "/_point1",
//         StateVarValue::Number(3.0),
//         StateVarValue::Number(1.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number1", "value", 1.0);
//     assert_sv_is_number(&dc, "/_number2", "value", 3.0);
//     assert_sv_array_is_number_list(&dc, "/_point1", "numericalXs", vec![3.0, 1.0]);
//     assert_sv_array_is_number_list(&dc, "/_point2", "numericalXs", vec![3.0, 4.0]);

//     move_point_2d(
//         &dc,
//         "/_point2",
//         StateVarValue::Number(1.0),
//         StateVarValue::Number(3.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number1", "value", 1.0);
//     assert_sv_is_number(&dc, "/_number2", "value", 1.0);
//     assert_sv_array_is_number_list(&dc, "/_point1", "numericalXs", vec![3.0, 1.0]);
//     assert_sv_array_is_number_list(&dc, "/_point2", "numericalXs", vec![1.0, 3.0]);
// }

// // =========== <number> ============

// #[wasm_bindgen_test]
// fn number_with_string_children() {
//     static DATA: &str = r#"
//     <number />
//     <number></number>
//     <number>5</number>
//     <number>5+1</number>
//     <number>5+ 1 </number>
//     <number>asfd</number>
//     <number> asdft + 5</number>

//     <!-- <number>5  1 </number> -->
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number1", "value", 0.0);
//     assert_sv_is_number(&dc, "/_number2", "value", 0.0);
//     assert_sv_is_number(&dc, "/_number3", "value", 5.0);
//     assert_sv_is_number(&dc, "/_number4", "value", 6.0);
//     assert_sv_is_number(&dc, "/_number5", "value", 6.0);
//     assert_sv_is_number(&dc, "/_number6", "value", f64::NAN);
//     assert_sv_is_number(&dc, "/_number7", "value", f64::NAN);
// }

// #[wasm_bindgen_test]
// fn number_invalid_children() {
//     static DATA: &str = r#"
//     <number><text>2</text></number>
//     <number><text>3 +</text><text>2</text></number>
//     <number>3 <text /></number>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let (_, warnings, errors) = doenet_core_from(DATA).unwrap();

//     assert_eq!(warnings.len(), 4);
//     assert!(warnings.contains(&DoenetMLWarning::InvalidChildType {
//         parent_comp_name: "/_number1".into(),
//         child_comp_name: "/_text1".into(),
//         child_comp_type: "text",
//         doenetml_range: RangeInDoenetML::None,
//     },));
//     assert!(warnings.contains(&DoenetMLWarning::InvalidChildType {
//         parent_comp_name: "/_number2".into(),
//         child_comp_name: "/_text2".into(),
//         child_comp_type: "text",
//         doenetml_range: RangeInDoenetML::None,
//     },));
//     assert!(warnings.contains(&DoenetMLWarning::InvalidChildType {
//         parent_comp_name: "/_number2".into(),
//         child_comp_name: "/_text3".into(),
//         child_comp_type: "text",
//         doenetml_range: RangeInDoenetML::None,
//     },));
//     assert!(warnings.contains(&DoenetMLWarning::InvalidChildType {
//         parent_comp_name: "/_number3".into(),
//         child_comp_name: "/_text4".into(),
//         child_comp_type: "text",
//         doenetml_range: RangeInDoenetML::None,
//     },));

//     assert_eq!(errors, vec![]);

// }

// #[wasm_bindgen_test]
// fn number_can_do_arithmetic_on_strings_and_number_children() {
//     static DATA: &str = r#"
//     <p>
//     <number>3 + 2 - 4 * 5</number>
//     <number copySource='/_number1'/>
//     <number copySource='/_number2'/>
//     <number copySource='/_number3'/>
//     </p>
    
//     <!-- Arithmetic in nested number -->
//     <p>
//     <number name='nested1'><number copySource='/_number1' copyProp='value'/></number>
//     <number name='nested2'>$nested1.value - 6</number>
//     <number name='nested3'>1.5 * $nested2.value</number>
//     <number name='nested4'>$nested3.value + $nested2.value + 1</number>
//     </p>
    
//     <!-- Arithmetic in nested numbers combined with copyProp value -->
//     <p>
//     <number name='combined1'>$nested1 + 3 * <number copySource='nested1' copyProp='value'/> + 1</number>
//     <number name='combined2'>$combined1 + 1</number>
//     <number name='combined3'>$combined2 / $combined2</number>
//     </p>
    
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "/_number1", "value", -15.0);
//     assert_sv_is_number(&dc, "/_number2", "value", -15.0);
//     assert_sv_is_number(&dc, "/_number3", "value", -15.0);
//     assert_sv_is_number(&dc, "/_number4", "value", -15.0);

//     assert_sv_is_number(&dc, "nested1", "value", -15.0);
//     assert_sv_is_number(&dc, "nested2", "value", -21.0);
//     assert_sv_is_number(&dc, "nested3", "value", -31.5);
//     assert_sv_is_number(&dc, "nested4", "value", -51.5);

//     assert_sv_is_number(&dc, "combined1", "value", -59.0);
//     assert_sv_is_number(&dc, "combined2", "value", -58.0);
//     assert_sv_is_number(&dc, "combined3", "value", 1.0);
// }

// #[wasm_bindgen_test]
// fn number_invalid_prop_index_does_not_crash() {
//     static DATA: &str = r#"
//     <sequence name='s' from='3' to='5' />   

//     <p><number name='num1' copySource='s' copyProp='value' propIndex = '100' /></p>
//     <p><number name='num2' copySource='s' copyProp='value' propIndex = '-23' /></p>
//     <p><number name='num3' copySource='s' copyProp='value' propIndex = 'asdf' /></p>
//     <p><number name='num4' copySource='s' copyProp='value' propIndex = '2.3' /></p>

//     <!-- This one should be valid -->
//     <p><number name='num5' copySource='s' copyProp='value' propIndex = '3.000' /></p>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     assert_eq!(warnings.len(), 3);
//     for warning in warnings {
//         assert!(matches!(
//             warning,
//             DoenetMLWarning::PropIndexIsNotPositiveInteger { .. }
//         ));
//     }

//     assert_eq!(errors, vec![]);

//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "num1", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num2", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num3", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num4", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num5", "value", 5.0);
// }

// #[wasm_bindgen_test]
// fn number_invalid_dynamic_prop_index_does_not_crash() {
//     static DATA: &str = r#"
//     <sequence name='s' from='3' to='5' />

//     <number name='n1'>100</number>
//     <number name='n2'>-23</number>
//     <number name='n3'>asdf</number>
//     <number name='n4'>2.3</number>
//     <number name='n5'>3.0000</number>

//     <number name='num1' copySource='s' copyProp='value' propIndex = '$n1' />
//     <number name='num2' copySource='s' copyProp='value' propIndex = '$n2' />
//     <number name='num3' copySource='s' copyProp='value' propIndex = '$n3' />
//     <number name='num4' copySource='s' copyProp='value' propIndex = '$n4' />

//     <!-- This one should be valid -->
//     <number name='num5' copySource='s' copyProp='value' propIndex = '$n5' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "num1", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num2", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num3", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num4", "value", f64::NAN);
//     assert_sv_is_number(&dc, "num5", "value", 5.0);
// }

// #[wasm_bindgen_test]
// fn number_parses_arithmetic_from_number_input_immediate_value() {
//     static DATA: &str = r#"
//     <numberInput/>
//     <numberInput copySource='/_numberInput1' />
//     <numberInput name='myNumInput' copySource='/_numberInput2' />

//     <number name='n1'>$myNumInput.immediateValue + 1</number>

//     <number name='n2' copySource='n1' />
//     <number name='n3' copySource='n2' />
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     update_immediate_value_for_number(&dc, "/_numberInput1", "5.0");
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_number(&dc, "n1", "value", 6.0);
//     assert_sv_is_number(&dc, "n2", "value", 6.0);
//     assert_sv_is_number(&dc, "n3", "value", 6.0);
// }

// // ========= <sources> ===========

// #[wasm_bindgen_test]
// fn sources_with_no_children() {
//     static DATA: &str = r#"
//     <sources></sources>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);
// }

// // ========= Macros ===========

// // // This test takes a long time to run
// // #[wasm_bindgen_test]
// // fn macro_prop_index_inside_prop_index_with_whitespace() {
// //     static DATA: &str = r#"
// //     <sequence hide name='s1' from='11' to='30' />
// //     <sequence hide name='s2' from='51' to='100' />
// //     <sequence hide name='s3' from='101' to='500' />

// //     <text>$s1[1].value</text>
// //     <text>$s2[$s1[3].value].value</text>
// //     <text>$s2[   $s1[3].value    ].value</text>
// //     <text>$s2[$s1[3].value    ].value</text>
// //     <text>$s2[ $s1[3].value].value</text>
// //     <text>$s3[ $s2[$s1[5].value].value ].value</text>
// //     <number>$s3[ $s2[$s1[ $s1[2].value ].value].value ].value</number>
// //     "#;
// //     display_doenet_ml_on_failure!(DATA);
// //     let dc = doenet_core_with_no_warnings_errors(DATA);
// //     doenet_core::update_renderers(&dc);

// //     assert_sv_is_string(&dc, "/_text1", "value", "11");
// //     assert_sv_is_string(&dc, "/_text2", "value", "63");
// //     assert_sv_is_string(&dc, "/_text3", "value", "63");
// //     assert_sv_is_string(&dc, "/_text4", "value", "63");
// //     assert_sv_is_string(&dc, "/_text5", "value", "63");
// //     assert_sv_is_string(&dc, "/_text6", "value", "165");
// //     assert_sv_is_number(&dc, "/_number1", "value", 172.0);
// // }

// // TODO: Do we want to allow this notation?
// // This test takes a long time to run
// #[wasm_bindgen_test]
// fn macro_prop_index_inside_prop_index_with_whitespace_older_notation() {
//     static DATA: &str = r#"
//     <sequence hide name='s1' from='11' to='30' />
//     <sequence hide name='s2' from='51' to='100' />
//     <sequence hide name='s3' from='101' to='500' />
    
//     <text>$s1.value[1]</text>
//     <text>$s2.value[$s1.value[3]]</text>
//     <text>$s2.value[   $s1.value[3]    ]</text>
//     <text>$s2.value[$s1.value[3]    ]</text>
//     <text>$s2.value[ $s1.value[3]]</text>
//     <text>$s3.value[ $s2.value[$s1.value[5]] ]</text>
//     <number>$s3.value[ $s2.value[$s1.value[ $s1.value[2] ]] ]</number>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "/_text1", "value", "11");
//     assert_sv_is_string(&dc, "/_text2", "value", "63");
//     assert_sv_is_string(&dc, "/_text3", "value", "63");
//     assert_sv_is_string(&dc, "/_text4", "value", "63");
//     assert_sv_is_string(&dc, "/_text5", "value", "63");
//     assert_sv_is_string(&dc, "/_text6", "value", "165");
//     assert_sv_is_number(&dc, "/_number1", "value", 172.0);
// }


// #[wasm_bindgen_test]
// fn preserve_space_between_macros() {
//     static DATA: &str = r#"
//         <text name='h'>hello</text>
//         <text name='w'>world</text>
//         <text name='hsw'>$h $w!</text>
//         <text name='hw'>$h$w!</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "hsw", "value", "hello world!");
//     assert_sv_is_string(&dc, "hw", "value", "helloworld!");
// }

// #[wasm_bindgen_test]
// fn number_after_dollar_is_not_macro() {
//     static DATA: &str = r#"
//         <text name='t'>It costs $1.</text>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_is_string(&dc, "t", "value", "It costs $1.");
// }

// #[wasm_bindgen_test]
// fn macros_fail_with_warnings() {
//     static DATA: &str = r#"
//     <text name="w">word</text>

//     <p><text name="t1">w: $w</text></p>
//     <p><text name="t2">w[1]: $w[1]</text></p>
//     <p><text name="t3">w.value: $w.value</text></p>
//     <p><text name="t4">w.foo: $w.foo</text></p>
//     <p><text name="t5">w.value[1]: $w.value[1]</text></p>
//     <p><text name="t6">w.foo[1]: $w.foo[1]</text></p>
//     <p><text name="t7">w[1].value: $w[1].value</text></p>
    
//     <p><text name="t8">x: $x</text></p>
//     <p><text name="t9">x[1]: $x[1]</text></p>
//     <p><text name="t10">x.foo: $x.foo</text></p>
//     <p><text name="t11">x.foo[1]: $x.foo[1]</text></p>
//     <p><text name="t12">x[1].foo: $x[1].foo</text></p>
    
//     <p><text name="t13">w[1: $w[1</text></p>
//     <p><text name="t14">w.1: $w.1</text></p>
    
//     <point name="P" xs="1 2" />
    
//     <p>P.xs[1]: <number name="n1">$P.xs[1]</number></p>
//     <p>P.xs[2]: <number name="n2">$P.xs[2]</number></p>
//     <p><text name="t15">P.xs[1: $P.xs[1</text></p>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let (dc, warnings, errors) = doenet_core_from(DATA).unwrap();
//     doenet_core::update_renderers(&dc);

//     assert_eq!(warnings.len(), 10);

//     let mut n_component = 0;
//     let mut n_statevar = 0;
//     let mut n_array_index = 0;
//     let mut n_other = 0;

//     for warning in warnings.iter() {
//         match warning {
//             DoenetMLWarning::ComponentDoesNotExist { comp_name: _, doenetml_range: _ } => {
//                 n_component += 1;
//             }
//             DoenetMLWarning::StateVarDoesNotExist { comp_name: _, sv_name: _, doenetml_range: _ } => {
//                 n_statevar += 1;
//             }
//             DoenetMLWarning::InvalidArrayIndex { comp_name: _, sv_name: _, array_index: _, doenetml_range: _ } => {
//                 n_array_index +=1;
//             }
//             _ => {
//                 n_other += 1;
//             }
//         }
//     }

//     assert_eq!(n_component, 5);
//     assert_eq!(n_statevar, 2);
//     assert_eq!(n_array_index, 3);
//     assert_eq!(n_other, 0);


//     assert_eq!(errors, vec![]);

//     assert_sv_is_string(&dc, "t1", "value", "w: word");
//     assert_sv_is_string(&dc, "t2", "value", "w[1]: ");
//     assert_sv_is_string(&dc, "t3", "value", "w.value: word");
//     assert_sv_is_string(&dc, "t4", "value", "w.foo: ");
//     assert_sv_is_string(&dc, "t5", "value", "w.value[1]: ");
//     assert_sv_is_string(&dc, "t6", "value", "w.foo[1]: ");
//     assert_sv_is_string(&dc, "t7", "value", "w[1].value: ");
//     assert_sv_is_string(&dc, "t8", "value", "x: ");
//     assert_sv_is_string(&dc, "t9", "value", "x[1]: ");
//     assert_sv_is_string(&dc, "t10", "value", "x.foo: ");
//     assert_sv_is_string(&dc, "t11", "value", "x.foo[1]: ");
//     assert_sv_is_string(&dc, "t12", "value", "x[1].foo: ");
//     assert_sv_is_string(&dc, "t13", "value", "w[1: $w[1");
//     assert_sv_is_string(&dc, "t14", "value", "w.1: $w.1");
//     assert_sv_is_string(&dc, "t15", "value", "P.xs[1: $P.xs[1");
//     assert_sv_is_number(&dc, "n1", "value", 1.0);
//     assert_sv_is_number(&dc, "n2", "value", 2.0);


// }

// // ========= Reloading essential data ============

// #[wasm_bindgen_test]
// fn reload_essential_data_after_point_moves() {
//     static DATA: &str = r#"
//     <number name='num'>2</number>
//     <graph name='g'>
//         <point name='p' xs='3 $num'/>
//     </graph>
//     "#;
//     display_doenet_ml_on_failure!(DATA);

//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p", "xs", vec![3.0, 2.0]);

//     move_point_2d(
//         &dc,
//         "p",
//         StateVarValue::Integer(5),
//         StateVarValue::Number(1.0),
//     );
//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p", "xs", vec![5.0, 1.0]);

//     let (dc, possible_warnings, errors) = doenet_core_with_essential_data(DATA, dc.essential_data).unwrap();
//     assert_eq!(possible_warnings.len(), 0);

//     assert_eq!(errors, vec![]);


//     doenet_core::update_renderers(&dc);

//     assert_sv_array_is_number_list(&dc, "p", "xs", vec![5.0, 1.0]);
// }

// // =============== Render tree ===================

// #[wasm_bindgen_test]
// fn render_tree_formats_state_vars_correctly() {
//     static DATA: &str = r#"
//     <document></document>
//     "#;
//     display_doenet_ml_on_failure!(DATA);
//     let dc = doenet_core_with_no_warnings_errors(DATA);
//     let render_tree_string = doenet_core::update_renderers(&dc);
//     let render_tree: serde_json::Value =
//         serde_json::from_str(&render_tree_string).expect("Render tree is not valid json.");
//     let components_list = render_tree
//         .as_array()
//         .expect("Render tree was not a list at the top level");
//     assert_eq!(components_list.len(), 1, "Render tree is incorrect length");

//     let component_data = components_list[0]
//         .as_object()
//         .expect("Render tree component data was not json object");

//     assert_eq!(
//         component_data.get("componentName"),
//         Some(&serde_json::Value::String("/_document1".into()))
//     );

//     let state_vars = component_data
//         .get("stateValues")
//         .expect("Render tree has no stateValues field for component")
//         .as_object()
//         .expect("Render tree stateValues is not a json object");

//     // Pick one state var of each type and test the formatting
//     assert_eq!(
//         state_vars.get("disabled"),
//         Some(&serde_json::Value::Bool(false)),
//         "Render tree boolean state var incorrect"
//     );
//     assert_eq!(
//         state_vars.get("creditAchieved"),
//         Some(&serde_json::Value::Number(
//             serde_json::Number::from_f64(1.0).unwrap()
//         )),
//         "Render tree number state var incorrect"
//     );
//     assert_eq!(
//         state_vars.get("submitLabel"),
//         Some(&serde_json::Value::String("Check Work".into())),
//         "Render tree string state var incorrect"
//     );
// }

// // Make sure that the $n variable name is not var0
// // <number name='n'>3.1</number>
// // <math name='m'>var + $n</math>
