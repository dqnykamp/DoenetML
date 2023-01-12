use super::*;
use crate::base_definitions::*;


use crate::ComponentProfile;

use lazy_static::lazy_static;



lazy_static! {
    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {
        use StateVarUpdateInstruction::*;

        vec![

            ("value",StateVarVariant::String(StateVarDefinition {

                dependency_instructions: vec![DependencyInstruction::Child {
                    desired_profiles: vec![ComponentProfile::Text],
                    parse_into_expression: false,
                }],
                

                determine_state_var_from_dependencies: |dependency_values| {
                    let textlike_children = &dependency_values[0];
                    DETERMINE_STRING(textlike_children).map(|x| SetValue(x))
                },

                ..Default::default()
            })),


            ("text", TEXT_DEFAULT_DEFINITION()),

            ("hidden", HIDDEN_DEFAULT_DEFINITION()),
            ("disabled", DISABLED_DEFAULT_DEFINITION()),
            ("fixed", FIXED_DEFAULT_DEFINITION()),

        ]

    };
}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "text",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,

        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "hide",
            "disabled",
            "fixed",
        ],

        primary_input_state_var_ind: Some(0),

        component_profiles: vec![
            (ComponentProfile::Text, "value")
        ],

        valid_children_profiles: ValidChildTypes::ValidProfiles(vec![ComponentProfile::Text]),
        
        ..Default::default()
    };
}
