use super::*;
use crate::base_definitions::*;


use lazy_static::lazy_static;



lazy_static! {
    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {

        vec![

        ("hidden", HIDDEN_DEFAULT_DEFINITION()),
        ("disabled", DISABLED_DEFAULT_DEFINITION()),

        ]
    };
}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "p",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "hide",
            "disabled",
        ],

        should_render_children: true,

        display_errors: true,
        
        action_names: || vec!["recordVisibilityChange"],

        valid_children_profiles: ValidChildTypes::AllComponents,
        
        ..Default::default()
    };
}
