use lazy_static::lazy_static;

use crate::state_variables::*;
use crate::base_definitions::*;

use super::*;



lazy_static! {

    pub static ref MY_STATE_VAR_DEFINITIONS: Vec<(StateVarName, StateVarVariant)> = {
        use StateVarUpdateInstruction::*;

        vec![
        
        ("submitLabel", StateVarVariant::String(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue("Check Work".to_string())),
            for_renderer: true,
            ..Default::default()
        })),

        ("submitLabelNoCorrectness", StateVarVariant::String(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue("Submit Response".to_string())),
            for_renderer: true,
            ..Default::default()
        })),

        ("hidden", StateVarVariant::Boolean(Default::default())),

        ("disabled", DISABLED_DEFAULT_DEFINITION()),

        ("fixed", FIXED_DEFAULT_DEFINITION()),

        // ("titleChildName", StateVarVariant::String(StateVarDefinition {
        //     determine_state_var_from_dependencies: |_| Ok(SetValue("Submit Response")),
        //     for_renderer: true,
        //     ..Default::default()
        // })),

        
        ("title", StateVarVariant::String(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue("".to_string())),
            for_renderer: true,
            ..Default::default()
        })),

        ("level", StateVarVariant::Number(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue(0.0)),
            for_renderer: true,
            ..Default::default()
        })),

        ("justSubmitted", StateVarVariant::Boolean(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue(true)),
            for_renderer: true,
            ..Default::default()
        })),

        ("showCorrectness", StateVarVariant::Boolean(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue(true)),
            for_renderer: true,
            ..Default::default()
        })),

        ("creditAchieved", StateVarVariant::Number(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue(1.0)),
            for_renderer: true,
            ..Default::default()
        })),

        ("createSubmitAllButton", StateVarVariant::Boolean(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue(false)),
            for_renderer: true,
            ..Default::default()
        })),


        ("suppressAnswerSubmitButtons", StateVarVariant::Boolean(StateVarDefinition {
            determine_state_var_from_dependencies: |_| Ok(SetValue(false)),
            for_renderer: true,
            ..Default::default()
        })),

        ]
    };
}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "document",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,
        
        state_var_index_map: MY_STATE_VAR_DEFINITIONS.iter().enumerate().map(|(i,v)| (v.0,i) ).collect(),

        attribute_names: vec![],

        should_render_children: true,

        display_errors: true,

        valid_children_profiles: ValidChildTypes::AllComponents,

        ..Default::default()
    };
}
