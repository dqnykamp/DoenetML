use std::collections::HashMap;

use super::*;
use crate::base_definitions::*;

use crate::ComponentProfile;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MY_STATE_VAR_DEFINITIONS: HashMap<StateVarName, StateVarVariant> = {
        use StateVarUpdateInstruction::*;

        let mut state_var_definitions = HashMap::new();

        state_var_definitions.insert("start_index", integer_definition_from_attribute!("start_index", 0));
        state_var_definitions.insert("end_index", integer_definition_from_attribute!("end_index", 0));

        state_var_definitions.insert(
            "message",
            StateVarVariant::String(StateVarDefinition {
                return_dependency_instructions: |_| {
                    let instruction = DependencyInstruction::Child {
                        desired_profiles: vec![ComponentProfile::Text],
                        parse_into_expression: false,
                    };

                    HashMap::from([("children_value_svs", instruction)])
                },

                determine_state_var_from_dependencies: |dependency_values| {
                    let error_message = dependency_values.dep_value("children_value_svs")?
                        .has_exactly_one_element()?
                        .into_string()?;
                    Ok(SetValue(error_message))
                },

                for_renderer: true,

                ..Default::default()
            }),
        );

        return state_var_definitions;
    };
}

lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "_error",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,

        attribute_names: vec![
            "start_index",
            "end_index",
        ],
        ..Default::default()
    };
}
