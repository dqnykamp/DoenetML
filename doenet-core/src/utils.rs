
use crate::component::*;
use crate::ComponentName;
use crate::ComponentNode;
use crate::StateVarMutableView;
// use crate::state::EssentialStateVar;
// use crate::EssentialDataOrigin;

use serde_json::{json, Map, Value};

/// Macros for logging.
macro_rules! log {
    ( $( $t:tt )* ) => {

        #[cfg(feature = "web")]
        web_sys::console::log_1(&format!( $( $t )* ).into());

        #[cfg(not(feature = "web"))]
        println!( $( $t )* )
    }
}
macro_rules! log_json {
    ( $label:expr, $a:expr ) => {
        #[cfg(feature = "web")]
        web_sys::console::log_2(
            &$label.into(),
            &wasm_bindgen::JsValue::from_serde(&$a).unwrap(),
        );
    };
}
macro_rules! log_debug {
    ( $( $t:tt )* ) => {

        // #[cfg(all(feature = "web", feature = "web-debug-log"))]
        #[cfg(feature = "web")]
        web_sys::console::debug_1(&format!( $( $t )* ).into());

        // #[cfg(not(feature = "web"))]
        // println!( $( $t )* )
    }
}

pub(crate) use log;
pub(crate) use log_debug;
pub(crate) use log_json;

/// List components and children in a JSON array
pub fn json_components(
    components: &Vec<ComponentNode>,
    component_states: &Vec<Vec<StateVarMutableView>>,
) -> Value {
    let json_components: Map<String, Value> = components
        .iter()
        .map(|component| {
            (
                component.name.to_string(),
                package_subtree_as_json(components, component_states, component),
            )
        })
        .collect();

    Value::Object(json_components)
}

pub fn package_subtree_as_json(
    components: &Vec<ComponentNode>,
    component_states: &Vec<Vec<StateVarMutableView>>,
    component: &ComponentNode,
) -> Value {
    let children: Map<String, Value> = component
        .children
        .iter()
        .enumerate()
        .map(|(child_num, child)| match child {
            ComponentChild::Component(comp_child_ind) => {
                let comp_child = &components[*comp_child_ind];
                (
                    format!("{} {}", child_num, comp_child_ind),
                    package_subtree_as_json(components, component_states, comp_child),
                )
            }
            ComponentChild::String(str) => {
                (format!("{}", child_num), Value::String(str.to_string()))
            }
        })
        .collect();

    let mut my_json_props: Map<String, Value> = Map::new();

    my_json_props.insert("children".to_owned(), json!(children));
    my_json_props.insert(
        "parent".to_owned(),
        match component.parent {
            Some(parent_ind) => Value::String(components[parent_ind].name.clone().into()),
            None => Value::Null,
        },
    );
    my_json_props.insert(
        "type".to_owned(),
        Value::String(component.definition.component_type.to_string()),
    );
    my_json_props.insert(
        "copySource".to_owned(),
        match &component.copy_source {
            Some(CopySource::Component(component_relative)) => {
                Value::String(format!("{:?}", component_relative))
            }
            Some(CopySource::StateVar(component_slice_relative)) => {
                Value::String(format!("{:?}", component_slice_relative))
            }
            None => Value::Null,
        },
    );

    for (static_attr_name, static_attr_val) in component.static_attributes.iter() {
        my_json_props.insert(
            format!("static attr {}", static_attr_name),
            Value::String(static_attr_val.to_string()),
        );
    }

    let component_state = &component_states[component.ind];

    for (state_var_name, state_var_ind) in component.definition.state_var_index_map.iter() {
        let state_for_state_var = &component_state[*state_var_ind];
        my_json_props.insert(
            format!("sv: {}", state_var_name),
            serde_json::Value::from(state_for_state_var),
        );
    }

    Value::Object(my_json_props)
}

// pub fn json_dependencies(
//     dependencies: &HashMap<DependencyKey, Vec<Dependency>>,
// ) -> HashMap<String, Vec<Dependency>> {

//     dependencies
//         .iter()
//         .map(|(k, deps)| {
//             (format!("{:?}", k), deps.clone())
//         })
//         .collect()
// }

// pub fn json_essential_data(
//     essential_data: &HashMap<ComponentName, HashMap<EssentialDataOrigin, EssentialStateVar>>,
// ) -> HashMap<String, HashMap<String, EssentialStateVar>> {
//     essential_data.iter().map(|(comp, h)|
//         (comp.clone(),
//              h.iter().map(|(origin, var)| {
//                 let origin_name = format!("From {:?}", origin);
//                 (origin_name, var.clone())
//              }).collect::<HashMap<String, EssentialStateVar>>()
//         )
//     ).collect()
// }
