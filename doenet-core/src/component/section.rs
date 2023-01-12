use std::collections::HashMap;

use super::*;
use crate::base_definitions::*;


use lazy_static::lazy_static;



lazy_static! {
    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {

        use StateVarUpdateInstruction::*;

        vec![

        ("submitLabel", string_definition_from_attribute!("submitLabel", "")),
        ("submitLabelNoCorrectness", string_definition_from_attribute!("submitLabelNoCorrectness", "")),
        ("boxed", boolean_definition_from_attribute!("boxed", false)),

        ("titleChildName", StateVarVariant::String(StateVarDefinition{
            ..Default::default()
        })),

        ("titlePrefix", StateVarVariant::String(StateVarDefinition{
            ..Default::default()
        })),

        ("title", StateVarVariant::String(StateVarDefinition{
            ..Default::default()
        })),

        ("containerTag", StateVarVariant::String(StateVarDefinition{
            determine_state_var_from_dependencies: |_| Ok(SetValue("section".to_string())),
            ..Default::default()
        })),

        ("level", StateVarVariant::Integer(StateVarDefinition{
            ..Default::default()
        })),

        ("justSubmitted", StateVarVariant::Boolean(StateVarDefinition{
            ..Default::default()
        })),

        ("showCorrectness", StateVarVariant::Boolean(StateVarDefinition{
            ..Default::default()
        })),

        ("creditAchieved", StateVarVariant::Integer(StateVarDefinition{
            ..Default::default()
        })),

        ("collapsible", StateVarVariant::Boolean(StateVarDefinition{
            ..Default::default()
        })),

        ("open", StateVarVariant::Boolean(StateVarDefinition{
            dependency_instructions: USE_ESSENTIAL_DEPENDENCY_INSTRUCTION(),
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
            initial_essential_value: true,
            ..Default::default()
        })),

        ("suppressAnswerSubmitButtons", StateVarVariant::Boolean(StateVarDefinition{
            ..Default::default()
        })),

        ("createSubmitAllButton", StateVarVariant::Boolean(StateVarDefinition{
            ..Default::default()
        })),

        ("hidden", HIDDEN_DEFAULT_DEFINITION()),
        ("disabled", DISABLED_DEFAULT_DEFINITION()),

        ]
    };
}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "section",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![
            "submitLabel",
            "submitLabelNoCorrectness",
            "boxed",

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
