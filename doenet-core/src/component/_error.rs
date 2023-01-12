use std::collections::HashMap;

use super::*;
use crate::base_definitions::*;

use crate::ComponentProfile;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {
        use StateVarUpdateInstruction::*;

        vec![

        ("start_index", integer_definition_from_attribute!("start_index", 0)),
        ("end_index", integer_definition_from_attribute!("end_index", 0)),

        (
            "message",
            StateVarVariant::String(StateVarDefinition {
                dependency_instructions: vec![
                    DependencyInstruction::Child {
                        desired_profiles: vec![ComponentProfile::Text],
                        parse_into_expression: false,
                    }
                ],


                determine_state_var_from_dependencies: |dependency_values| {
                    let error_message = dependency_values[0][0].into_string()?;
                    Ok(SetValue(error_message))
                },

                for_renderer: true,

                ..Default::default()
            }),
        )
        ]

    };
}

lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "_error",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "start_index",
            "end_index",
        ],
        ..Default::default()
    };
}
