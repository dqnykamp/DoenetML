use std::collections::HashMap;

use lazy_static::lazy_static;

use super::*;
use crate::base_definitions::*;



lazy_static! {

    pub static ref MY_STATE_VAR_DEFINITIONS: HashMap<StateVarName, StateVarVariant> = {

        use StateVarUpdateInstruction::*;

        let mut state_var_definitions = HashMap::new();

        state_var_definitions.insert("value", StateVarVariant::String(StateVarDefinition {
            return_dependency_instructions: |_|
                HashMap::from([
                    ("essential", DependencyInstruction::Essential { prefill: Some("prefill") }),
                    ("immediate", DependencyInstruction::StateVar {
                        component_name: None,
                        state_var_name: "immediateValue",
                    }),
                    ("sync", DependencyInstruction::StateVar {
                        component_name: None,
                        state_var_name: "syncImmediateValue",
                    }),
                ]),
            determine_state_var_from_dependencies: |dependency_values| {
                let essential_value = dependency_values.dep_value("essential")?
                    .has_exactly_one_element()?
                    .into_string()?;
                let immediate_value = dependency_values.dep_value("immediate")?
                    .has_exactly_one_element()?
                    .into_string()?;
                let sync_values = dependency_values.dep_value("sync")?
                    .has_exactly_one_element()?
                    .into_bool()?;

                let value =
                    if sync_values {
                        immediate_value
                    } else {
                        essential_value
                    };
                Ok(SetValue(value))
            } ,
            request_dependencies_to_update_value: |desired_value, sources| {
                HashMap::from([
                    ("essential", Ok(vec![
                        DependencyValue {
                            source: sources.get("essential").unwrap().first().unwrap().0.clone(),
                            value: desired_value.clone().into(),
                        }
                    ])),
                    ("sync", Ok(vec![
                        DependencyValue {
                            source: sources.get("sync").unwrap().first().unwrap().0.clone(),
                            value: StateVarValue::Boolean(true),
                        }
                    ])),
                    ("immediate", Ok(vec![
                        DependencyValue {
                            source: sources.get("immediate").unwrap().first().unwrap().0.clone(),
                            value: desired_value.into(),
                        }
                    ])),
                ])
            },
            ..Default::default()
        }));

        state_var_definitions.insert("immediateValue", StateVarVariant::String(StateVarDefinition {
            for_renderer: true,
            return_dependency_instructions: |_|
                HashMap::from([
                    ("essential", DependencyInstruction::Essential { prefill: Some("prefill") }),
                ]),
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
            ..Default::default()
        }));

        state_var_definitions.insert("syncImmediateValue", StateVarVariant::Boolean(StateVarDefinition {
            return_dependency_instructions: USE_ESSENTIAL_DEPENDENCY_INSTRUCTION,
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
            initial_essential_value: true,
            ..Default::default()
        }));



        state_var_definitions.insert("expanded", StateVarVariant::Boolean(StateVarDefinition {
            for_renderer: true,
            determine_state_var_from_dependencies: |_| Ok(SetValue(false)),
            ..Default::default()
        }));

        state_var_definitions.insert("size", StateVarVariant::Number(StateVarDefinition {


            determine_state_var_from_dependencies: |_| {
                Ok(SetValue(10.0))
            },
            for_renderer: true,
            ..Default::default()
        }));

        state_var_definitions.insert("width", StateVarVariant::Number(StateVarDefinition {
            for_renderer: true,
            determine_state_var_from_dependencies: |_| Ok(SetValue(600.0)),
            ..Default::default()
        }));

        state_var_definitions.insert("hidden", HIDDEN_DEFAULT_DEFINITION());
        state_var_definitions.insert("disabled", DISABLED_DEFAULT_DEFINITION());

        return state_var_definitions
    };


}



lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        component_type: "textInput",

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,

        attribute_names: vec![
            "hide",
            "disabled",
            "prefill",
        ],

        action_names: || vec!["updateImmediateValue", "updateValue"],

        on_action: |action_name, args, resolve_and_retrieve_state_var| {
            match action_name {
                "updateImmediateValue" => {
                    // Note: the key here is whatever the renderers call the new value
                    let new_val = args.get("text").expect("No text argument").first().unwrap();

                    vec![
                        ("immediateValue", new_val.clone()),
                        ("syncImmediateValue", StateVarValue::Boolean(false)),
                    ]
                },

                "updateValue" => {

                    let new_val = resolve_and_retrieve_state_var(&"immediateValue")
                        .unwrap().try_into().unwrap();
                    let new_val = StateVarValue::String(new_val);

                    vec![
                        ("value", new_val),
                        ("syncImmediateValue", StateVarValue::Boolean(true)),
                    ]

                }

                _ => panic!("Unknown action '{}' called on textInput", action_name)
            }
        },

        ..Default::default()
    };
}
