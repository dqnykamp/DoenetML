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
        
        ("value", StateVarVariant::Number(StateVarDefinition {
            for_renderer: true,

            dependency_instructions: vec![
                DependencyInstruction::Child {
                    desired_profiles: vec![ComponentProfile::Number],
                    parse_into_expression: true,
                }
            ],

            determine_state_var_from_dependencies: |dependency_values| {
                let children = &dependency_values[0];

                // let (expression, numerical_values) = split_dependency_values_into_math_expression_and_values(children)?;

                match DETERMINE_NUMBER(children) {
                    Ok(x) => Ok(SetValue(x)),
                    Err(msg) => {
                        crate::utils::log!("Error determing number: {}", msg);
                        Ok(SetValue(f64::NAN))
                    },
                }
            },

            request_dependencies_to_update_value: |desired_value, dependency_sources| {
                let children_sources = &dependency_sources[0];
                vec![
                    (0, DETERMINE_NUMBER_DEPENDENCIES(desired_value, children_sources))
                ]
            },

            ..Default::default()
        })),

        ("text", StateVarVariant::String(StateVarDefinition {
            for_renderer: true,

            dependency_instructions: vec![
                DependencyInstruction::StateVar {
                    component_name: None,
                    state_var_name: "value",
                }
            ],

            determine_state_var_from_dependencies: |dependency_values| {

                let value: Option<f64> = dependency_values[0].get(0)
                    .into_if_exists()?;

                Ok(SetValue(
                    value.map_or("".to_string(), |val| val.to_string())
                ))

            },

            ..Default::default()
        })),

        ("hidden", HIDDEN_DEFAULT_DEFINITION()),
        ("disabled", DISABLED_DEFAULT_DEFINITION()),

        ]

    };
}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "number",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "hide",
            "disabled",
        ],

        primary_input_state_var_ind: Some(0),

        component_profiles: vec![
            (ComponentProfile::Number, "value"),
            (ComponentProfile::Text, "value"),
        ],

        valid_children_profiles: ValidChildTypes::ValidProfiles(vec![ComponentProfile::Number]),
                
        ..Default::default()
    };
}
