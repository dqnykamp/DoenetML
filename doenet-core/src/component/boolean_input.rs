use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::state_variables::*;
use crate::base_definitions::*;
use super::*;

use crate::ComponentProfile;



lazy_static! {
    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {

        vec![
            ("value", StateVarVariant::Boolean(StateVarDefinition {
        
            dependency_instructions: USE_ESSENTIAL_DEPENDENCY_INSTRUCTION,
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,

            for_renderer: true,

            ..Default::default()
        })),

        ("hidden", HIDDEN_DEFAULT_DEFINITION()),

        ("disabled", DISABLED_DEFAULT_DEFINITION()),
        ]
    };
}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "booleanInput",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "hide",
            "disabled",
        ],

        primary_input_state_var_ind: Some(0),

        component_profiles: vec![
            (ComponentProfile::Boolean, "value")
        ],

        action_names: || vec!["updateBoolean"],

        on_action: |action_name, args, _| {
            match action_name {
                "updateBoolean" => {

                    let new_val = args.get("boolean").expect("No boolean argument").first().unwrap();

                    vec![(
                        0,
                        new_val.clone()
                    )]
                }

                _ => panic!("Unknown action '{}' called on booleanInput", action_name)
            }
        },

        ..Default::default()
    };
}
