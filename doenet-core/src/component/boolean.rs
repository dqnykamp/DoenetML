use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::state_variables::*;
use crate::base_definitions::*;

use super::*;

use crate::ComponentProfile;



lazy_static! {
    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {
        use StateVarUpdateInstruction::*;

        vec![
        
        ("value", StateVarVariant::Boolean(StateVarDefinition {

            dependency_instructions: vec![
                DependencyInstruction::Child {
                    desired_profiles: vec![ComponentProfile::Boolean, ComponentProfile::Text],

                    parse_into_expression: true,
                }
            ],

            determine_state_var_from_dependencies: |dependency_values| {
                let children = &dependency_values[0];
                DETERMINE_BOOLEAN(children).map(|x| SetValue(x))
            },
            for_renderer: true,
            ..Default::default()
        })),

        ("hidden", HIDDEN_DEFAULT_DEFINITION()),
        ("text", TEXT_DEFAULT_DEFINITION()),

        ]
    };
}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "boolean",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "hide",
        ],

        primary_input_state_var_ind: Some(0),

        component_profiles: vec![
            (ComponentProfile::Boolean, "value"),
            // (ComponentProfile::Text, "value"),
        ],

        valid_children_profiles: ValidChildTypes::ValidProfiles(vec![
            ComponentProfile::Number,
            ComponentProfile::Boolean,
        ]),

        ..Default::default()
    };
}
